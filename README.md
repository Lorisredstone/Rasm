# Rasm

Rasm est un language approximatif, codé en rust et qui se compile en assmbleur x86_64.

Non, je ne connais ni le rust ni l'assembleur, mais oui je me lance la dedans, on verra bien  

## Comment lancer rasm ?

Pour compiler et lancer `input/input.pyasm`

```bash
cargo build && ./target/debug/rasm -f input/input.pyasm && ./output/output
```

Pour lancer les tests

```bash
python3 test.py -t
```

Pour enregistrer de nouveaux tests

```bash
python3 test.py -r
```

Pour juste compiler rasm

```bash
cargo build --release
```

## Objectifs long terme

- [ ] Turing complet
- [x] Accès mémoire correct
- [ ] Self hosting
- [ ] Type checking

## Objectifs proches

- [ ] système d'erreurs
- [ ] tests qui lancent rasm et non pas l'output
- [ ] tests capable de gérer les erreurs
- [x] macros
- [x] includes de fichiers
- [ ] mode interprété
- [x] mode compilé
- [ ] mode debug
- [x] syscalls
- [x] système de mémoire
- [ ] stdlib (refaire celle du C?)
- [x] découper rasm en plusieurs fichiers
- [ ] Détailler plus la syntaxe dans le readme
- [x] Ignorer les commentaires
- [x] Syscall => push eax/rax
- [x] Over
- [x] Fonctions séparées de mem
- [x] Divmod
- [x] Rot
- [x] Mémoire 8/16/32/64 bits
- [x] Fonctions sans modifier la stack
- [ ] Gérer les booléens
- [ ] Tests insensible \n \r\n
- [ ] Basic memory allocator
- [ ] If a function is not used, dont compile it
- [x] Else
- [ ] Gérer les tests qui ne terminent jamais comme une erreur

## Syntaxe

### Découpage

La syntaxe sera découpée en trois morceaux :

- L'explication de la commande
- Un cas d'utilisation dans pyasm
- Un équivalent en python

### `<int>`

Push le nombre donné sur la stack

```dart
3
```

```py
stack.push(3)
```

### `.`

Affiche le nombre sur la stack en décimal

```dart
3 .
```

```py
stack.push(3)
print(stack.pop())
```

### `+`

Additionne les deux éléments sur la stack et push le résultat

```dart
3 3 +
```

```py
stack.push(stack.pop() + stack.pop())
```

### `*`

Multiplie les deux éléments sur la stack et push le résultat

```dart
3 3 *
```

```py
stack.push(stack.pop() * stack.pop())
```

### `-`

Soustrait le premier element de la stack par le deuxième

```dart
3 4 -
```

```py
a = stack.pop()
b = stack.pop()
stack.push(a - b)
```

### `divmod`

Calcule le reste et le dividende

```dart
5 10 divmod
```

```py
nombre = stack.pop()
diviseur = stack.pop()
reste = nombre % diviseur
dividende = nombre // diviseur
stack.push(dividende)
stack.push(reste)
```

## Inspirations et remerciments

- [Porth](https://gitlab.com/tsoding/porth) Pour l'idée d'un language compilé en assembleur
- [@tsodingDaily](https://www.youtube.com/@TsodingDaily) Pour m'avoir donné envie de créer un language, puis un autre, puis un autre...
- [@rchapman](https://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/) Pour ce magnifique blog sur les syscall linux
