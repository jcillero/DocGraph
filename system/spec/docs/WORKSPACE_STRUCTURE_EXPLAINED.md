# WORKSPACE_STRUCTURE_GUIDE
Guía estructural del workspace portable

Versión: 1.0.0  
Naturaleza: Documentación explicativa (alineada con la Definición Operativa)  
Ámbito: Estructura física del workspace  

---

# 1️⃣ Propósito

Este documento explica la función y naturaleza de cada carpeta del workspace.

No sustituye a la Definición Operativa.
No redefine invariantes.
No introduce reglas nuevas.

Su objetivo es facilitar:

- Comprensión rápida del árbol
- Ubicación correcta de nuevos elementos
- Prevención de errores estructurales
- Uso consistente en futuras aplicaciones

---

# 2️⃣ Principio General

Cada carpeta del workspace tiene una **naturaleza principal**:

- Normativa
- Declarativa
- Runtime
- Tooling
- Regenerable
- Entregable Final

Ninguna carpeta debe cambiar de naturaleza sin evaluación formal.

---

# 3️⃣ Nivel Raíz

## START_APP.bat
Lanzador principal.
No contiene lógica de dominio.
Selecciona modo CLI/UI según configuración.

## USER_MANUAL.md
Manual orientado a usuario final.
No contiene reglas arquitectónicas.

---

# 4️⃣ Carpeta `system/`

Naturaleza: **Normativa**  
Modificable en runtime: No  
Contiene ejecutables: Sí (binarios embebidos)

Es el núcleo estable del sistema.

Incluye:

- Runtimes embebidos
- Entry points
- Configuración normativa
- Declaraciones spec
- Assets estáticos

Nunca debe modificarse durante ejecución normal.

---

## 4.1 `system/assets/`

Naturaleza: Normativa  
Contiene:

- Imágenes
- Iconos
- Logos
- Plantillas
- Recursos gráficos

No contiene lógica ni configuración.

Ejemplo:

```

system/assets/images/reports/
system/assets/images/ui/

```

---

## 4.2 `system/bin/`

Naturaleza: Materializada (runtime)

Contiene:

- runtimes embebidos (python, llm, etc.)
- entrypoints
- tools productivas
- utilidades externas embebidas

No debe contener:
- Documentación
- Especificaciones declarativas
- Tooling de desarrollo

---

## 4.3 `system/config/`

Naturaleza: Configuración normativa

Contiene:

- runtime_registry.json
- entrypoints.json
- system_policy.json
- llm_provider.json

Es la autoridad para:

- Resolución de runtimes
- Entry points
- Configuración estructural

No modificable en runtime.

---

## 4.4 `system/spec/`

Naturaleza: Declarativa

No contiene código ejecutable.

Incluye:

- Esquemas
- Catálogos
- Declaraciones de scripts
- Documentación normativa
- Vistas derivadas

Su función es describir el sistema, no ejecutarlo.

---

# 5️⃣ Carpeta `user/`

Naturaleza: Runtime (mutable)

Zona de operación del usuario.

Puede contener:

- inputs
- outputs
- projects
- runs
- config de usuario

Nunca debe contener:

- Código ejecutable del sistema
- Configuración normativa

---

## 5.1 `user/inputs/`
Entradas de trabajo.

## 5.2 `user/outputs/`
Entregables finales.

## 5.3 `user/projects/`
Organización por proyecto.

## 5.4 `user/runs/`
Trazabilidad por ejecución.
No se permite sobrescritura.

## 5.5 `user/config/`
Preferencias de usuario.
No pueden alterar invariantes.

---

# 6️⃣ Carpeta `dev/`

Naturaleza: Tooling

No forma parte del runtime productivo.

Puede eliminarse sin romper la aplicación.

Contiene:

- Scripts de desarrollo
- Reportes técnicos
- Validaciones
- Planes
- Logs de construcción

No debe contener:
- Código requerido para ejecución normal
- Recursos productivos

---

# 7️⃣ Carpeta `trash/`

Naturaleza: Temporal / Regenerable

Espacio controlado para:

- Archivos temporales
- Residuos de pruebas
- Material descartable

No forma parte del contrato arquitectónico.

---

# 8️⃣ Reglas de Ubicación

Antes de añadir un archivo, responder:

1. ¿Es normativo o mutable?
2. ¿Es declarativo o ejecutable?
3. ¿Es runtime o tooling?
4. ¿Es regenerable o final?

Ejemplos:

| Tipo | Ubicación |
|------|-----------|
| Script productivo | system/bin/tools/ |
| Script dev | dev/scripts/ |
| Imagen de portada | system/assets/images/ |
| Informe generado | user/outputs/ |
| Log de ejecución | user/runs/<tool>/ |
| Esquema JSON | system/spec/schemas/ |

---

# 9️⃣ Errores Estructurales Comunes

❌ Poner imágenes en `user/` cuando son parte del producto  
❌ Poner scripts dev en `system/bin/`  
❌ Escribir en `system/` desde runtime  
❌ Mezclar outputs finales con runs regenerables  
❌ Hardcodear rutas absolutas  

---

# 🔟 Relación con la Definición Operativa

Este documento:

- No redefine invariantes
- No introduce nuevas entidades
- No altera gobernanza

Solo explica cómo se materializa físicamente la arquitectura declarada.

En caso de conflicto, prevalece la Definición Operativa.

---

# 🎯 Conclusión

La estructura del workspace no es arbitraria.

Es parte del contrato arquitectónico.

Mantener su coherencia garantiza:

- Portabilidad real
- Escalabilidad
- Auditoría posible
- Reducción de deuda técnica
- Reutilización como plantilla base

Este documento actúa como guía práctica para preservar esa coherencia.
```

---


