---
uuid: "86a2f359-8137-43e4-b5ff-61d235ef3cde"
name: "nextjs-hydration-client-state"
version: "1.0.0"
nature: "tactical-norm"
author: "norm-creator"
scope: "frontend"
category: "architecture"
dependencies: []
---

## Directriz Core

En componentes `"use client"` pre-renderizados en servidor (Next.js App Router), el HTML del servidor debe coincidir con el **primer render del cliente** (antes de `useEffect`). Si el primer render del cliente usa APIs solo disponibles en navegador (`localStorage`, `sessionStorage`, `window` con datos distintos al SSR) para **decidir qué árbol mostrar**, React produce hydration mismatch (`Expected server HTML to contain a matching …`).

### Anti-patrones (provocan divergencia)

1. Leer `localStorage` / `sessionStorage` en el cuerpo del render (incluido `typeof window !== "undefined" ? localStorage.getItem(...) : null`) para ramificar UI.
2. Inferir locale o ruta desde `window.location` en render cuando el SSR usa otra fuente (p. ej. `next-intl` / middleware).
3. Usar `typeof window === "undefined"` como única guarda de “solo cliente”; en cliente el primer render sigue debiendo igualar al servidor.

### Patrones recomendados

| Situación | Patrón |
|-----------|--------|
| Storage para decidir UI antes de hidratar | `useState` + `useEffect` o hook `useHasMounted`; no ramificar por storage hasta `hasMounted === true`. Primer render (servidor + cliente) usa el mismo estado inicial. |
| Suscripción al storage con paridad SSR | `useSyncExternalStore` con `getServerSnapshot` que devuelve el mismo valor que el primer render del cliente (p. ej. `null` o valor por defecto). |
| Locale / traducciones | `useLocale()` de `next-intl`, no el primer segmento de `window.location.pathname`. |
| Datos async en formularios | `values` de `react-hook-form` o `reset()` cuando cambien props; no solo `defaultValues` si los datos llegan después del mount. |

Implementar un hook canónico `useHasMounted` (primer render `false`, tras montaje `true`) y usarlo antes de leer storage para ramas de UI que deben coincidir con SSR. `suppressHydrationWarning` solo como último recurso; preferir paridad real.

## Restricciones Duras (Aduana de Fricción)

- Prohibido leer `localStorage`, `sessionStorage` o ramificar UI por su valor en el cuerpo del render de un componente `"use client"` pre-renderizado en servidor.
- Prohibido usar `window.location` en render para decidir locale, ruta o contenido visible cuando el SSR resuelve esos valores por otra vía (middleware, `next-intl`, props de servidor).
- Prohibido confiar únicamente en `typeof window === "undefined"` para evitar mismatch; el primer frame del cliente debe ser idéntico al HTML del servidor.
- Prohibido mostrar árboles de componentes distintos servidor/cliente en el primer paint sin patrón `useHasMounted`, `useSyncExternalStore` con `getServerSnapshot`, o equivalente con paridad demostrable.
- Prohibido usar `suppressHydrationWarning` como sustituto sistemático de paridad SSR/cliente sin justificación técnica documentada.
- Prohibido usar `defaultValues` estáticos en formularios cuando los datos iniciales llegan de forma asíncrona post-mount sin `reset()` o `values` controlados.
