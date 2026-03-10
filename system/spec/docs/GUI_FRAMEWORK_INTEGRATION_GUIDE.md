# GUI_FRAMEWORK_INTEGRATION_GUIDE
Integración GUI (CustomTkinter) + Scripts Estandarizados

Versión: 1.0.0  
Naturaleza: Documento estratégico–operativo  
Ámbito: Construcción de aplicaciones portables con GUI autogenerada  

---

# 1️⃣ Objetivo

Definir cómo convertir un conjunto de scripts Python estructurados en:

- Aplicación portable
- GUI modular (CustomTkinter)
- Autogenerada desde metadatos
- 100% offline
- Auditable y trazable

El GUI no contiene lógica de dominio.
Solo orquesta ejecución y muestra resultados.

---

# 2️⃣ Principio Central

> El GUI describe.
> Los scripts ejecutan.
> El registry gobierna.

Nunca duplicar lógica de negocio en la interfaz.

---

# 3️⃣ Arquitectura Base

Estructura mínima recomendada:

```

scripts/
ingest/
search/
redact/
ui/
config/
actions.json
ui.json
logs/
workspace/

```

Cada script es:

- Autónomo
- Ejecutable por CLI
- Auditable
- Reproducible

---

# 4️⃣ Contrato Obligatorio de Scripts

Todos los scripts deben cumplir:

## CLI estándar

Argumentos recomendados:

- --input
- --output
- --config
- --dry-run
- --verbose

## Salida

stdout → JSON limpio estructurado  
stderr → error < 400 caracteres  
exit code → estandarizado

Códigos recomendados:

0 → Éxito  
2 → Error validación  
3 → Dependencia ausente  
4 → Error E/S  
5 → Error interno  

## Logging

Cada ejecución debe registrar:

- timestamp
- acción
- input
- output
- exit
- meta

Archivo recomendado:

```

logs/ops.csv

```

---

# 5️⃣ actions.json (Interfaz Declarativa)

El GUI se genera automáticamente desde:

```

config/actions.json

````

Cada acción define:

- label (texto botón)
- cmd (comando CLI)
- ui.fields (campos de entrada)
- parámetros dinámicos {placeholder}

Ejemplo conceptual:

```json
{
  "MD:BUILD": {
    "label": "Compilar PDF",
    "cmd": ["python", "scripts/redact/md_build.py", "--input", "{md}"],
    "ui": {
      "fields": [
        {"name": "md", "type": "file", "label": "Markdown"}
      ]
    }
  }
}
````

Añadir una acción no requiere modificar el GUI.

---

# 6️⃣ GUI Autogenerado

El módulo GUI debe:

* Leer actions.json
* Crear pestañas dinámicamente
* Generar formularios según definición
* Ejecutar vía subprocess.run
* Mostrar stdout y stderr
* Actualizar logs

Componentes típicos:

* CTkTabview
* CTkFrame
* CTkButton
* CTkEntry
* CTkTextbox
* CTkProgressBar

---

# 7️⃣ Progreso en Tiempo Real

Para scripts largos:

* Emitir eventos JSONL por stdout:

  * start
  * progress
  * stage
  * done

El GUI:

* Interpreta eventos
* Actualiza barra de progreso
* Permite cancelación controlada

Separar:

* Progreso (stdout estructurado)
* Logs (archivo)

---

# 8️⃣ Buenas Prácticas

* Evitar rutas absolutas
* Diseñar scripts idempotentes
* Validar stdout como JSON antes de mostrar
* Probar con --dry-run antes de integrar
* Centralizar logs y hashes
* No bloquear GUI (usar threads o .after())

---

# 9️⃣ Riesgos y Mitigación

Scripts heterogéneos
→ Aplicar plantilla CLI base

Bloqueo GUI
→ Ejecutar en hilo separado

Salida inconsistente
→ Validar JSON antes de parsear

Rutas no portables
→ Normalizar rutas relativas

---

# 🔟 Beneficios del Enfoque

* GUI 5–10× más rápido de implementar
* Añadir nuevas acciones sin tocar interfaz
* Reducción de errores humanos
* Trazabilidad completa
* Compatibilidad offline real
* Escalabilidad modular

---

# 11️⃣ Nivel de Madurez

Este modelo permite:

Nivel 1 → Scripts + CLI
Nivel 2 → GUI autogenerado
Nivel 3 → Logs + hashes + auditoría
Nivel 4 → Aplicación enterprise offline auditada

---

# 🎯 Conclusión

El sistema GUI + Scripts no es una capa visual.
Es una arquitectura declarativa:

* Scripts estandarizados
* Metadatos describen interfaz
* GUI ejecuta sin reinterpretar
* Logs y hashes garantizan trazabilidad
* Operación 100% offline

Este modelo permite construir aplicaciones portables complejas
sin duplicar lógica y sin perder control arquitectónico.

```

---

