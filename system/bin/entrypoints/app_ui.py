"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "app_ui.py",
  "version": "1.0.0",
  "type": "entrypoint",
  "category": "ui",
  "description": "Punto de entrada de la interfaz gráfica. Resuelve la raíz de la aplicación, crea MainWindow y arranca el bucle principal de la UI sin incluir lógica de dominio.",
  "location_expected": "system/bin/entrypoints/",
  "entry_point": "main",
  "inputs": [
    {
      "type": "filesystem_context",
      "name": "app_root",
      "description": "Raíz de la aplicación portable resuelta desde la ubicación del propio script."
    }
  ],
  "outputs": [
    {
      "type": "ui_session",
      "description": "Instancia de la ventana principal y sesión interactiva de interfaz gráfica."
    }
  ],
  "dependencies": [
    "system.app.ui.app.main_window.MainWindow"
  ],
  "side_effects": [
    "start_ui_event_loop"
  ],
  "modifies_system": false,
  "modifies_user": false,
  "modifies_dev": false,
  "requires_runtime": "python",
  "runtime_version": ">=3.10",
  "deterministic": true,
  "idempotent": true,
  "notes": [
    "No debe contener lógica de dominio.",
    "No debe escribir directamente en user/runs ni user/output.",
    "La resolución de app_root depende de la posición estructural estable del archivo dentro del árbol portable."
  ]
}
================================================================================
"""

import sys
from pathlib import Path

from system.app.ui.app.main_window import MainWindow


def main() -> int:

    # raíz de la aplicación
    app_root = Path(__file__).resolve().parents[3]

    window = MainWindow(app_root=app_root)
    window.mainloop()

    return 0


if __name__ == "__main__":
    sys.exit(main())