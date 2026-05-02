# Principio Constitucional: Cimiento Arquitectónico

## Propósito
Establecer la estructura física y lógica fundamental del proyecto, asegurando escalabilidad, mantenibilidad y preparación para futuras integraciones.

## Definición
Este principio dicta la organización del código en capas claras y la adopción de patrones de diseño robustos.

## Directrices
1. **Jerarquía Física:** El código reside en `src/`, con directorios claramente separados (`app/`, `components/`, `lib/`, `contexts/`, `types/`).
2. **Composición:** Los componentes se construyen por composición; los hooks encapsulan lógica reutilizable.
3. **Contratos:** Las interacciones con la API externa se definen mediante tipos TypeScript y clientes HTTP centralizados en `src/lib/api/`.

## Estado Actual
- **En GesFer.Admin.Front:** Ubicación en `src/` (Next.js 14 App Router). Proyecto frontend independiente.
- **Composición:** Componentes UI primitivos + Shared compuestos + Layouts.
- **Contratos:** Tipos en `src/types/` y `src/lib/types/`; clientes API en `src/lib/api/`.
