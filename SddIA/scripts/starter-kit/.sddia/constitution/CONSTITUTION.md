# Constitución del proyecto (plantilla producto)

> **nature: product** — Sustituir por la constitución real del repositorio cliente. Ejemplo basado en origen legacy SddIA_1.

## 1. Definición de la arquitectura
El proyecto se rige por un principio de desacoplamiento entre su lógica central y sus métodos de presentación.

### 1.1. Core (producto)
Es el motor central del **software que construyes** (dominio cliente), no confundir con el Core **SddIA** (motor IA inyectado bajo `SddIA/`).

### 1.2. Implementaciones de interfaz
Capas externas que sirven de puente entre el usuario y el Core de producto.

---

## 2. Reglas de oro de jerarquía
1. Independencia del Core de producto respecto a capas de presentación cuando sea posible.
2. Dirección de dependencias unidireccional hacia el núcleo de dominio.
3. Preparar el sistema para nuevos puntos de entrada sin reescribir el núcleo.

---

## 3. Trazabilidad SddIA (motor)
Los cambios normativos o estructurales bajo `./SddIA/` (dependencia inyectada) deben seguir el protocolo de evolución del motor (`SddIA/evolution/`), independiente de la evolución de producto en `docs/evolution/` o rutas de documentación del cliente. Ver `SddIA/agents/cumulo.instructions.json` (separación de naturalezas).
