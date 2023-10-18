from typing import Optional
from click import command, option
from rust_over_pyo3 import compute


@command()
@option('--add', 'command', flag_value='add')
@option('--sub', 'command', flag_value='sub')
@option('--mul', 'command', flag_value='mul')
@option('--arg1', help='First complex number in the form x+yj')
@option('--arg2', help='Second complex number in the form x\'+y\'j')
def cli(command: Optional[str], arg1: Optional[str], arg2: Optional[str]) -> None:
    n1: complex = complex(arg1)
    n2: complex = complex(arg2)
    result: complex = compute(command, n1, n2)
    print(result)


if __name__ == '__main__':
    cli()
