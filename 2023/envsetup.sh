<<<<<<< HEAD

export LESS=R

c() { clear; }
b() { cargo --color always build; }
=======
c() { clear; }
b() { cargo build; }
>>>>>>> fa9437d (Chromebook file)
re() { target/debug/app example;  }
re1() { target/debug/app example1;  }
re2() { target/debug/app example2; }
ri() { target/debug/app input;  }
<<<<<<< HEAD

wb() {
    inotifywait -q -m -e close_write src/main.rs |
    while read -r filename event; do
        b |& less
        if [[ -e .wbstop ]]; then
            rm -r .wbstop
            exit 0
        fi
    done
}
=======
>>>>>>> fa9437d (Chromebook file)
