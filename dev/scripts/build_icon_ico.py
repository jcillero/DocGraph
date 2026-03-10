"""
================================================================================
SCRIPT_META (NO MODIFICAR FORMATO)
================================================================================
{
  "script_name": "build_icon_ico.py",
  "version": "1.1.1",
  "type": "dev_tooling",
  "category": "assets",
  "description": "Genera un archivo .ico multi-resolución a partir de un PNG maestro.",
  "location_expected": "dev/scripts/",
  "runtime_required": false,
  "modifies_system": false,
  "output_location": "system/assets/images/logos/",
  "outputs": [
    "docgraph_icon_active.ico"
  ],
  "depends_on": [
    "Pillow"
  ],
  "alignment_required": [
    "DEFINICIÓN OPERATIVA",
    "app_root BASE"
  ]
}
================================================================================
"""
import argparse
from pathlib import Path
from PIL import Image


DEFAULT_SIZES = [16, 24, 32, 48, 64, 128, 256, 512]


def parse_args():
    parser = argparse.ArgumentParser(description="Build multi-resolution ICO.")
    parser.add_argument("--input", required=True, help="Ruta PNG de entrada (relativa al root)")
    parser.add_argument("--output", required=True, help="Ruta ICO de salida (relativa al root)")
    parser.add_argument(
        "--sizes",
        default=",".join(str(s) for s in DEFAULT_SIZES),
        help="Lista de tamaños separados por coma (ej: 16,32,64)"
    )
    return parser.parse_args()


def _parse_sizes(sizes_str: str) -> list[tuple[int, int]]:
    raw = [s.strip() for s in sizes_str.split(",") if s.strip()]
    vals: list[int] = []
    for s in raw:
        try:
            v = int(s)
            if v > 0:
                vals.append(v)
            else:
                print(f"⚠️ Tamaño no válido (<=0) ignorado: {s}")
        except ValueError:
            print(f"⚠️ Tamaño inválido ignorado: {s}")

    vals = sorted(set(vals))
    return [(v, v) for v in vals]


def main():
    args = parse_args()
    root = Path(".").resolve()

    input_path = root / args.input
    output_path = root / args.output

    if not input_path.exists():
        print(f"❌ No se encuentra el archivo: {input_path}")
        return

    if output_path.suffix.lower() != ".ico":
        print(f"⚠️ El output no termina en .ico: {output_path.name}")

    sizes = _parse_sizes(args.sizes)
    if not sizes:
        print("❌ No hay tamaños válidos en --sizes")
        return

    # Asegura carpeta destino
    output_path.parent.mkdir(parents=True, exist_ok=True)

    # Asegura alpha estable
    img = Image.open(input_path).convert("RGBA")
    img.save(output_path, sizes=sizes)

    print("✅ ICO generado correctamente")
    print(f"   input : {input_path}")
    print(f"   output: {output_path}")
    print(f"   sizes : {', '.join(str(w) for (w, _h) in sizes)}")


if __name__ == "__main__":
    main()