# PLX
### **P**ractice programming exos in a delightful **L**earning e**X**perience

**STATUT: conception pour le projet PDG en HES d'été**

Au lieu de faire un projet juste pour "passer PDG", investissons ces 3 semaines à développer quelque chose qui peut avoir un fort impact sur l'expérience d'apprentissage à la HEIG-VD sur le long terme ! Si le projet atteint un état assez fonctionnel, il pourra déjà être utilisé 2 semaines plus tard à la rentrée sur des exercices de PRG1, PCO et SYE!

> *The age of features is over, we are living in the age of experiences.*  
> Aral Balkan, [dans une conférence sur l'UX intitulée "Superheroes & Villains in Design"](https://small-tech.org/videos/ux-talk/).

Ce n'est pas juste un projet stylé parce que le Rust c'est hype, qu'il y a un mode watch super réactif, un feedback riche... **on développe une nouvelle expérience d'apprentissage pour s'approcher de la pratique délibérée en informatique !!**

## Pourquoi
Les exercices de code sont au coeur de l'apprentissage d'un language de programmation, cependant avoir des exercices avec des petits programmes ou fonctions à implémenter ne garantit pas que l'expérience de pratique sera efficiente. Selon la pratique délibérée, l'apprentissage profond passe par une boucle de feedback la plus courte possible, or l'expérience actuelle est loin d'être fluide et efficace.

Prenons l'exemple d'un exercice un *petit programme en C qui demande le prénom et l'âge* et affiche une phrase incluant ces 2 valeurs. L'exo fourni dans un PDF, inclut une consigne, un bout de code de départ et un exemple d'exécution, ainsi qu'un code de solution sur la page suivante.  
Pour résoudre l'exercice, une fois la consigne lue, nous allons ouvrir un IDE, créer un fichier `main.c` manuellement, copier-coller le code de départ, lire le code existant et compléter les parties à développer.  
Une fois terminé, passons à la compilation en ouvrant un terminal dans l'IDE en tapant `gcc main main.cpp & main`, euh zut c'était `gcc -o main main.cpp && ./main`, on rentre prénom et age, puis comparons l'output manuellement pour voir si c'est bien le résultat attendu, réouvrons la consigne et non il manque l'affichage de l'âge! Revenons au code, on ajoute l'âge et on relance le build et l'exécution, on rentre prénom et âge à nouveau. Est-ce que l'output est bon cette fois ? Vérifions maintenant notre code avec la solution. Okay, on aurait pu utiliser `printf` au lieu de 2 fois `puts()` pour afficher le nom complet. Passons à l'exo suivant, cherchons sa consigne, la voilà, on recommence le cycle,...

Tous ces petites étapes supplémentaires autour de la rédaction du code semblent insignifiantes à première vue mais leur cumul résulte en une friction générale importante. En plus, il n'y aura que peu d'exécutions manuels c'est-à-dire très peu d'occasions de connaître la progression et d'ajuster son code au fur et à mesure, en plus d'une petite charge mentale pour compiler et lancer à la main.

Imaginons que dans un laboratoire de C *nous développions maintenant une bataille navale dans le terminal*. Tester de bout en bout de manière automatique un programme en C n'est pas une tâche évidente, en partie par manque d'outil adapté. Pour tester le fonctionnement global, il faut manuellement lancer une partie et jouer plusieurs coups pour vérifier le fonctionnement et vérifier à chaque étape si le jeu est cohérent dans son état et affichage. Une fois qu'une partie du jeu fonctionne, en développant le reste on risque de casser d'autres parties sans s'en rendre compte.

Un dernier cas concret, en *développant un petit shell en C++*, pour tester l'implémentation des pipes, il faudra compiler le shell ainsi que les CLIs accessibles, lancer le shell, puis taper quelques commandes du type `echo hey there | toupper` voir si l'output est bien `HEY THERE`, ce qui est très lent! Tester plein de plein de cas limites (plusieurs pipes, symbole de pipe collé, redirection stdout et non stderr, exit du CLI à droite du pipe, ...)

En résumé, le manque de validation automatisée ralentit le développement et l'apprentissage. Simplement ajouter des tests automatisés ne résoud pas tous les problèmes, car les tests runner ne sont pas adaptés à des tests sur des exos (pas de consigne, pas d'indices, affichage pas adapté, pas de mode watch, ...), il manque une partie d'automatisation autour. De plus, le travail d'écriture de tests pour des tous petits exos serait beaucoup trop conséquent, dans plein de cas comparer l'output avec une solution suffit à estimer si le programme est fonctionnel.

## Expérience PLX
**Le défi est d'arriver à réduire la friction au strict minimum, d'automatiser toutes les étapes administratives et de fournir un feedback riche, automatique et rapide durant l'entrainement.**

Cette expérience sera atteinte via
1. **La suppression des étapes de compilation et d'exécution manuelles**  
Aucune connaissance du système de compilation ou de ses commandes n'est nécessaire, tout se fait automatiquement dès qu'un des fichiers est sauvé (il suffit donc de taper Ctrl+S ou d'attendre que l'IDE sauve automatiquement)
1. **La suppression de toutes les rédactions manuelles de valeurs dans le terminal**  
Permettre de définir des arguments du programme et un contenu à injecter en `stdin`, avec des variantes pour tester différents cas.
1. **La suppression des étapes de comparaison d'output**  
L'output sera automatiquement comparé et une diff précise (avec surlignage des différences sur chaque ligne) sera affichée pour voir immédiatemment les différences. La diff pourrait supporter du trimming de l'output ou des lignes afin d'ignorer certains espaces blancs insignifiants. Les retours à la ligne et tabulations seront affichées avec un symbole visible.
1. **Une affichage et comparaison avec solution**  
Une fois l'exo résolu, pouvoir auto évaluer sa réponse avec la solution d'un prof est déjà d'une grande aide. Il sera possible de voir une diff de sa réponse avec la solution directement dans PLX.
1. **Une transition fluide entre exos**  
Passer à l'exo suivant devrait prendre moins de 4 secondes, le temps de passer de l'IDE à PLX (Alt+Tab), d'un raccourci (n) dans PLX pour afficher l'exo suivant et le temps que l'IDE réagisse à la demande d'ouverture du fichier.
1. **Aucun changement de fenêtre durant l'exo**  
PLX à gauche avec toute la consigne, l'IDE à droite dans un seul fichier utile, une fois les 2 fenêtres ouvertes, il n'y a plus de changement à faire comme tout est déjà disponible. La consigne s'affiche dans PLX, dès que le fichier ouvert est sauvé, le build et l'exécution se relance. Les erreurs de build sont visibles ainsi que les résultats des tests.

## Contexte
Ce projet tire inspiration de [Rustlings](https://rustlings.cool/) qui permet de s'habituer aux erreurs du compilateur Rust en corrigeant des problèmes de compilation ou en complétant une centaine de petits exercices. Dans la même idée, d'autres languages ont suivis avec golings, cplings, ziglings, ... Ce même projet a inspirée [PRJS](https://github.com/samuelroland/prjs) (Practice Runner for JavaScript), développée à l'occasion du dernier labo libre de WEB et qui permet de s'entrainer sur des fonctions vérifiées via des tests unitaires écrits et lancés avec Vitest en arrière plan.

PLX pousse encore plus loin l'expérience en supportant plusieurs languages, en y incluant la compilation automatique ainsi que le support de types de tests plus primitifs et plus simple à mettre en place qu'avec un framework de test.

Note: contrairement à Rustlings, ce repository ne contient pas d'exercices réels, seulement le code de la TUI PLX. Des exercices de démonstration seront écrits dans différents languages dans un sous dossier `examples`.

## Types de tests
Tous les tests pourront inclure les arguments à passer au programme. Il y a cependant différent types de tests imaginés.

- **Output simples**  
Comparer l'output console d'un exo soit avec un bout de texte définie, ou si la solution est définie, directement avec son output (évitant de maintenir un output de solution séparé).
- **Output avec injection stdin**  
Pareil mais avec la possibilité d'injecter une string d'un coup ou par morceau (avec des délais d'attente) pour simuler des entrées utilisateurs.
- **External command output**  
Parfois l'output utile à comparer n'est pas celui du programme mais celui d'une commande externe. Par ex. si un programme doit créer 3 fichiers .txt, le critère de vérification pourrait être que l'output de `cat *.txt` soit le même qu'après avoir lancé le programme solution dans un autre dossier.
- **Test runner**  
Supporter les tests runner existants, ici le plus utilisé pour le C++: GoogleTest, en lançant ces tests runner en arrière plan, en générant un export des résultats et de les afficher dans PLX

## Format de fichier texte
Afin de décrire les exos (titre, consigne, les tests associés de tous types), des fichiers textes seront utilisés pour être facilement versionnable et éditable. Soit on invente un format de zéro, soit on définit juste un schéma dans un format existant (JSON, YAML, TOML, ...)

<!-- Voici un exemple de quelques idées de syntaxe pour se donner une idée -->
TODO: encore à définir...

## Détails sur l'idée de projet pour PDG 2024
Techniquement, PLX sera une TUI (Text User Interface) en Rust (probablement en utilisant un framework de TUI comme ratatui) développée avec une équipe de gens motivés par apprendre un peu le Rust cet été. Elle sera donc crossplateform pour permettre un usage sous Linux, Windows et Mac.

**Fonctionnalités à implémenter durant PDG**
Dans l'ordre de priorité. Le support du Java étant la dernière priorité et sera fait si le temps le permet.
1. Format de fichiers à définir, affiner, demander des feedbacks à des profs, tester de convertir différents types d'exos pour trouver les limites.
1. Choisir un outil de build ([xmake](https://xmake.io/#/) par ex.)
1. Implémenter le build et l'exécution de code C++, avec passage d'arguments
1. Implémentation du chargement et de la navigation dans les listes de compétences et exos
1. Implémenter un affichage des détails d'un exo durant l'entrainement
1. Implémenter un mode watch solide pour lancer build+run automatiquement
1. Support des tests sur l'output simple
1. Support de GoogleTest
1. Support du C
1. Support du Java

**Exercices disponibles**  
Le recueil de [PRG1](https://github.com/PRG1-HEIGVD/PRG1_Recueil_Exercices) et celui de [PRG2](https://github.com/PRG2-HEIGVD/PRG2_Recueil_Exercices), ainsi que les exos de PCO avec des suites de tests GoogleTest, sont des bonnes références sur laquel réfléchir à la conception, tenter de transformer dans un nouveau format, ...

**Conception**  
3 étudiants de PRG2 (cours de C) ont été interviewé et les problèmes exprimés plus haut se confirment. La vision générale a déjà été travaillée comme le montre ce README, mais il reste toute la conception détaillée pour définir à quoi va ressembler les listes d'exos et les détails de l'expérience de pratique: comment afficher les erreurs de compilation et de crash, supporter les couleurs, faire des belles diff, permettre de naviguer dans des grands outputs et voir le détail des tests avec beaucoup d'output, ...  
Il reste tout autant de considérations techniques: comment compiler facilement sans config (si possible) du C, C++ et du Java, afficher l'exécution au fur et à mesure, quel outil de diffing choisir, comment implémenter un mode watch efficace, gérer les compilations en arrière plan, récupérer le résultat de GoogleTest, parser les fichiers textes décrivant les exos, produire des binaires de la TUI pour les 3 OS...

**Equipe**  
Je souhaiterai travailler avec une équipe de 5 personnes motivées par cette vision de cette expérience PLX, si possible avec des bases de Rust ou l'envie d'apprendre/d'expérimenter cet été. Le but est aussi de faire des review de codes pour chaque PR et d'avoir des suite de tests, pour simplifier la maintenance à long terme et faciliter le développement bouillonnant sur 3 semaines.

## Usage à la HEIG-VD
En considérant le nombre important de cours qui propose déjà actuellement des exercices en C++ (PRG1, ASD, PCO), C (PRG2, SYE), Java (POO, DAI, POA), l'impact de cet outil et cette nouvelle expérience pourrait être assez large si les enseignants sont motivés à l'adopter.  
Le but est de limiter au maximum la quantité de travail de restructuration des exercices existants et que les configurations de compilation soient non nécessaires ou très simple à mettre en place. Je pense qu'il sera facile de motiver 2-3 étudiants pour aider à faire la migration de ces différents cours.

Si l'outil est utilisable et déployable à la rentrée 2024 et que les exercices peuvent être facilement et rapidement adaptés (transformer les fichiers Markdown existants pour PRG1 par ex.), les étudiants de PRG1 pourront déjà en bénéficier dès les premières semaines ! Si GoogleTest est supporté, les exos de PCO pourront aussi être entrainé avec PLX. En SYE, les labos dans SO3 pourraient aussi être aidé (beaucoup d'étudiants perdent du temps à lancer les commandes de build, à éxecuter, à tester des commandes).

## Licence
Ce projet n'a pas encore de licence libre, il est donc techniquement propriétaire pour le moment. Une fois le projet PDG terminé, le projet ne va pas s'arrêter, des finitions seront probablement nécessaires et la maintenance continuera sur le temps libre. Tout comme PRJS, l'objectif important est de demander à l'école de reprendre les droits d'auteurs afin de pouvoir publier ce projet sous licence libre. Une condition pour faire partie de l'équipe est donc d'être d'accord de cette finalité pour qu'au moment de faire les démarches administratives tout le monde soit déjà d'accord.

