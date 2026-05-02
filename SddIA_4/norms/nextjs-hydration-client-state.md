# Next.js App Router: hidratación y estado solo en cliente

## Problema

En componentes `"use client"` que **también se pre-renderizan en el servidor**, el HTML del servidor debe coincidir con el **primer render del cliente** (antes de `useEffect`).

Si el primer render del cliente usa APIs solo disponibles en el navegador (`localStorage`, `sessionStorage`, `window` con datos distintos al SSR) para **decidir qué árbol de componentes mostrar**, el HTML suele **no coincidir** con el generado en el servidor. React muestra advertencias del tipo:

`Warning: Expected server HTML to contain a matching <div> in <div>`

## Anti-patrones (provocan divergencia)

1. **Leer `localStorage` / `sessionStorage` en el cuerpo del render** (incluido `typeof window !== "undefined" ? localStorage.getItem(...) : null`) para ramificar UI: en el servidor no hay storage; en el cliente a veces sí, así que el primer paint del cliente puede diferir del servidor.
2. **Inferir locale o ruta desde `window.location` en render** cuando el SSR usa otra fuente (p. ej. `next-intl` / middleware): el primer frame del cliente puede diferir.
3. **`typeof window === "undefined"` como única guarda** para “solo cliente”: en el cliente el primer render sigue siendo un “primer frame” que debe igualar al servidor.

## Patrones recomendados

| Situación | Patrón |
| :--- | :--- |
| Necesitas storage para decidir UI antes de hidratar | `useState` + `useEffect` (o un hook **`useHasMounted`**) y **no** ramificar por storage hasta `hasMounted === true`. El primer render (servidor + cliente) usa el mismo estado inicial. |
| Necesitas suscripción al storage con paridad SSR | `useSyncExternalStore` con **`getServerSnapshot`** que devuelve el mismo valor que en el primer render del cliente (p. ej. `null` o valor por defecto). |
| Locale / traducciones en componentes | `useLocale()` de `next-intl`, no el primer segmento de `window.location.pathname`. |
| Datos async en formularios | `values` de `react-hook-form` o `reset()` cuando cambien las props, no solo `defaultValues` si los datos llegan después del mount. |

## Referencia en código

- Hook **`useHasMounted`** (`src/hooks/use-has-mounted.ts`): primer render `false`, tras montaje `true`; usar antes de leer storage para ramas de UI que deben coincidir con el SSR.
- **`ProtectedRoute`**: ejemplo de uso de `hasMounted` antes de interpretar `auth_user` en el árbol visible.

## Comprobaciones

- **Lint / tests:** véase `src/__tests__/policies/nextjs-hydration-policy.test.ts` (presencia de la norma y del hook canónicos).
- **Agente Tekton:** restricción en `SddIA/agents/tekton-developer.json` (paridad SSR / cliente en componentes cliente).

## Enlaces relacionados

- [React — suppressing hydration mismatches](https://react.dev/reference/react-dom/client/hydrateRoot#suppressing-unavoidable-hydration-mismatch-errors) (solo como último recurso; preferir paridad real).
- Documentación del bug de referencia: `docs/bugs/fix-mycompany-rastros-ui-i18n/technical-note-hydration.md`.
