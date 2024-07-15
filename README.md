# PLX
> **P**ractice programming exos in a delightful **L**earning e**X**perience

**STATUT: conception pour le projet PDG en HES d'été**

Au lieu de faire un projet juste pour "passer PDG", investissons ces 3 semaines à développer quelque chose qui peut avoir un fort impact sur l'expérience d'apprentissage à la HEIG-VD sur le long terme ! Si le projet atteint un état assez fonctionnel, il pourra déjà être utilisé dès la semaine suivante sur des exercices de PRG1, PCO et SYE!

## Pourquoi
Pour expérimenter un cours de programmation conçu autour de la pratique délibérée, les exercices sont au coeur de l'apprentissage, cependant avoir des exercices avec petits programmes ou fonctions à implémenter ne garantit pas que l'expérience de pratique sera efficiente. Selon la pratique délibérée, l'apprentissage profond passe par une boucle de feedback la plus courte possible.

Prenons l'exemple d'un exercice de C fourni dans un PDF, incluant une consigne, un bout de code de départ et un output attendu, ainsi qu'une solution sur la page suivante.  
Pour résoudre l'exercice, une fois la consigne lue, nous allons créer un fichier `main.c` manuellement, copier-coller le code de départ de la consigne, lire le code existant et compléter les parties à développer. Une fois terminé, passons à la compilation en ouvrant un terminal dans l'IDE en tapant `gcc main main.cpp & main`, euh zut c'était .`gcc -o main main.cpp && ./main`, puis comparons l'output manuellement pour voir si c'est bien le résultat attendu, réouvrons la consigne et non il manque une partie! Revenons au code, on corrige le soucis et on relance le build et l'exécution. Est-ce que l'output est bon cette fois ? La fin a l'air mieux mais est-ce que ces 20 lignes sont vraiment toutes correctes ? On va dire que c'est bon... Vérifions maintenant notre code avec la solution. Okay, on aurait pu utiliser `printf` au lieu de 2 fois `puts()` pour afficher le nom complet. Passons à l'exo suivant, cherchons sa consigne, la voilà,...

Tous ces petits étapes supplémentaires autour de la rédaction du code semblent insignifiantes à première vue mais leur cumul résulte en une friction générale importante. En plus, il y aura beaucoup moins d'exécutions manuels qu'avec des tests automatisé, c'est à dire beaucoup moins d'occasions de connaître la progression et d'ajuster son code.

Imaginons que dans un laboratoire de C nous développions maintenant une bataille navale dans le terminal. Tester de bout en bout de manière automatique un programme en C n'est pas une tâche évidente, surtout par manque d'outil adapté. Pour tester le fonctionnement global, il faut manuellement lancer une partie et jouer plusieurs coups pour vérifier le fonctionnement et vérifier à chaque étape si le jeu est cohérent et si l'affichage est correct. Une fois qu'une partie du jeu fonctionne, en développant le reste on risque de casser sans s'en rendre compte le reste.

Un dernier cas concret, en développant un petit shell en C++, pour tester l'implémentation des pipes, il faudra builder le shell ainsi que les CLIs accessibles, lancer le shell, puis lancer quelques commandes du type `echo hey there | toupper` voir si l'output est bien "HEY THERE".

En résumé, le manque de test automatisé ralentit le développement et l'apprentissage comme il n'y a pas de moyen de tout vérifier rapidement. Simplement ajouter des tests n'est pas suffisant, car les tests runner ne sont pas adaptés à des tests sur des exos, seulement du code de production, il manque une partie d'automatisation autour.

## Expérience PLX
**Le défi est d'arriver à réduire la friction au strict minimum, d'automatiser toutes les étapes administratives et de fournir un feedback riche, automatique et rapide sur l'état d'avancement.**

Cette expérience 

Le but est de mettre en place une expérience de pratique avec une boucle de feedback la plus courte possible.

## Contexte
Ce projet tire inspiration de [Rustlings](https://rustlings.cool/) qui permet d'apprendre de s'habituer aux erreurs du compilateur Rust en corrigeant des problèmes de compilation ou en complétant des petits exercices. Ce même projet a inspirée [PRJS](https://github.com/samuelroland/prjs) (Practice Runner for JavaScript) et qui permet de s'entrainer des fonctions en JavaScript vérifiées via des tests unitaires écrits et lancés avec Vitest en arrière plan. PRJS a été développé pour le dernier labo libre de WEB.

PLX pousse encore plus loin l'expérience en supportant plusieurs languages en y incluant la compilation automatique ainsi que des types de tests plus primitifs et simple à mettre en place qu'avec un framework de test comme Vitest en JS.

Note: contrairement à Rustlings, ce repository ne contient pas d'exercices réels, seulement le code du CLI PLX. Des exercices de démonstration seront écrits dans différents languages dans un sous dossier `examples`.

## Types de tests
Pour permettre

## Format texte
Afin de décrire les exos (titre, consigne, les tests associés de tous types), des fichiers textes seront utilisés pour être facilement versionnable et éditable. Soit on invente un format de texte de zéro, soit on définit juste un schéma dans un format existant (JSON, YAML, TOML, ...)

Voici un exemple de quelques idées de syntaxe pour se donner une idée


## Détails sur l'idée de projet pour PDG 2024
Techniquement, PLX sera une TUI (Text User Interface) en Rust (probablement en utilisant un framework de TUI comme ratatui) développée avec une équipe de gens motivés par apprendre un peu le Rust cet été. Elle sera donc crossplateform pour permettre un usage sous Linux, Windows et Mac.

**Fonctionnalités à implémenter durant PDG**
1. Format de fichiers à définir, affiner, demander des feedbacks à des profs, tester de convertir différents types d'exos pour trouver les limites.

3 étudiants de PRG2 ont été interviewé et les problèmes exprimés se confirment. La vision générale a déjà été travaillée comme le montre ce README, mais il reste toute la conception détaillée pour définir à quoi va ressembler les listes d'exos et les détails et l'expérience de pratique d'un seul exo (comment afficher les erreurs de build et de crash, comment supporter les couleurs, comment faire des belles diff, comment permettre de changer de mise en page, ...).  
Et tout autant de considérations techniques: comment builder facilement sans config si possible du C, C++ et du Java, comment afficher l'exécution au fur et à mesure, quel outil de diffing choisir, comment implémenter un mode watch efficace, comment gérer les builds en arrière plan, comment récupérer le résultat de GoogleTest et autres, comment parser les fichiers textes décrivant les exos, comment produire des binaires pour les 3 OS...

## Usage à la HEIG-VD
En considérant le nombre important de cours qui propose déjà actuellement des exercices en C++ (PRG1, ASD, PCO), C (PRG2, SYE), Java (POO, DAI, POA), l'impact de cet outil pourrait être assez large si les enseignants sont motivés à l'adopter.  
Le but est de limiter au maximum la quantité de travail de restructuration des exercices existants et que les configurations de build sont non nécessaires ou très simple à mettre en place. Il sera facile aussi de motiver 2-3 étudiants pour aider à faire la migration pour faire un peu de travail répétitif manuel.

Si l'outil est utilisable (pas besoin d'être forcément fini) et déployable à la rentrée 2024 et que les exercices peuvent être facilement et rapidement adaptés (transformer les fichiers Markdown existants pour PRG1 par ex.), les étudiants de PRG1 pourront déjà en bénéficier ! Si GoogleTest est supporté, les exos de PCO pourront aussi être entrainé avec PLX. En SYE, les labos dans SO3 seraient grandement aidé (beaucoup d'étudiants perdent du temps à lancer les commandes de build, à éxecuter, à tester des commandes).

