# Rulare locală

Acest pachet conține:

- `statement.pdf` - enunțul problemei;
- `public_blueprints/` - directorul cu testele publice (`north.in`, `south_01.in`, ...,
`south_12.in`);
- `interactor.cpp` - interactorul local;
- `run_local.py` - scriptul care compilează și rulează soluția împreună cu interactorul.

## Cerințe

Aveți nevoie de:

- Python 3;
- `g++/clang++/c++`, pentru compilarea interactorului;
- compilatorul sau runtime-ul potrivit pentru soluția voastră:

| Limbaj | Compilator/Runtime    | Parametri folosiți  |
|--------|-----------------------|---------------------|
| C      | `gcc`/`clang`/`cc`    | `-std=c11 -O2`      |
| C++    | `g++`/`clang++`/`c++` | `-std=c++17 -O2`    |
| Python | `python3`             | `-u`                |
| Java   | `javac` + `java`      |                     |
| Rust   | `rustc`               | `-O`                |

Scriptul suportă soluții scrise în C, C++, Python, Java și Rust.

## Comanda de bază

Puneți soluția voastră în același director cu fișierele din arhivă, și rulați:

```bash
python3 run_local.py
```

Implicit scriptul va rula `solution.cpp`. Pentru un alt fișier, precizați soluția
cu `-s` / `--solution`:

```bash
python3 run_local.py --solution solution.c
python3 run_local.py -s solution.py
python3 run_local.py --solution Solution.java
python3 run_local.py -s solution.rs
```

Mai întâi se compilează interactorul (din `interactor.cpp`); apoi scriptul va compila soluția dacă
a fost modificată, o va rula pe toate testele din `public_blueprints/` și va afișa scorul fiecărui test,
urmat de scorul total.

După fiecare test, interacțiunea completă dintre soluție și interactor este salvată automat
într-un fișier de log în `logs/<soluție>__<test>.log`.
Scorurile fiecărui test și scorul total sunt afișate în terminal.

## Rulare pe un singur test

Folosiți opțiunea `-t` / `--test` pentru a rula un singur fișier de test:

```bash
python3 run_local.py --test public_blueprints/north.in
python3 run_local.py -t public_blueprints/north.in -s solution.py
```

## Rulare pe un director de teste

Folosiți opțiunea `--test-dir` pentru a rula toate fișierele de test dintr-un director:

```bash
python3 run_local.py --test-dir public_blueprints/
```

## Rulare directă (fără compilare de către script)

Dacă vreți să personalizați comanda de rulare, puteți furniza direct comanda după `--`:

```bash
python3 run_local.py --test public_blueprints/north.in -- ./bpc_2026_solution_cpp
python3 run_local.py -t public_blueprints/south01.in -- python3 sol.py
python3 run_local.py -- ./bpc_2026_sol_c
```

Tot ce urmează după `--` este tratat ca și comanda de rulare a soluției.

> **Notă:** Pe Codeforces sunt suportate doar limbajele enumerate mai sus (C, C++, Python, Java,
> Rust), compilate cu parametrii specificați în tabelul de mai sus.

## Curățarea fișierelor generate

Pentru a șterge fișierele generate de script, rulați:

```bash
python3 run_local.py clean
```

## Observații

- Programul vostru trebuie să comunice prin `stdin` și `stdout`, conform protocolului din enunț.
- Nu uitați să faceți `flush` după fiecare comandă trimisă interactorului dacă modul în care
afișați la `stdout` nu face deja asta.
