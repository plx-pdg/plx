# PLX
> **P**ractice programming exos in a delightful **L**earning e**X**perience

**STATUT: conception pour le projet PDG en HES d'été**

Au lieu de faire un projet juste pour "passer PDG", investissons ces 3 semaines à développer quelque chose qui peut avoir un fort impact sur l'expérience d'apprentissage à la HEIG-VD sur le long terme ! Si le projet atteint un état assez fonctionnel, il pourra déjà être utilisé 2 semaines plus tard à la rentrée sur des exercices de PRG1, PCO et SYE!

## Pourquoi
Pour expérimenter un cours de programmation conçu autour de la pratique délibérée, les exercices sont au coeur de l'apprentissage, cependant avoir des exercices avec des petits programmes ou fonctions à implémenter ne garantit pas que l'expérience de pratique sera efficiente. Selon la pratique délibérée, l'apprentissage profond passe par une boucle de feedback la plus courte possible, or l'expérience actuelle est loin d'être fluide et efficace.

Prenons l'exemple d'un exercice de C fourni dans un PDF, incluant une consigne, un bout de code de départ et un output attendu, ainsi qu'une solution sur la page suivante.  
Pour résoudre l'exercice, une fois la consigne lue, nous allons créer un fichier `main.c` manuellement, copier-coller le code de départ de la consigne, lire le code existant et compléter les parties à développer.  
Une fois terminé, passons à la compilation en ouvrant un terminal dans l'IDE en tapant `gcc main main.cpp & main`, euh zut c'était .`gcc -o main main.cpp && ./main`, puis comparons l'output manuellement pour voir si c'est bien le résultat attendu, réouvrons la consigne et non il manque une partie! Revenons au code, on corrige le soucis et on relance le build et l'exécution. Est-ce que l'output est bon cette fois ? La fin a l'air mieux mais est-ce que ces 20 lignes sont vraiment toutes correctes ? On va dire que c'est bon... Vérifions maintenant notre code avec la solution. Okay, on aurait pu utiliser `printf` au lieu de 2 fois `puts()` pour afficher le nom complet. Passons à l'exo suivant, cherchons sa consigne, la voilà,...

Tous ces petites étapes supplémentaires autour de la rédaction du code semblent insignifiantes à première vue mais leur cumul résulte en une friction générale importante. En plus, il y aura beaucoup moins d'exécutions manuels qu'avec des tests automatisé, c'est à dire beaucoup moins d'occasions de connaître la progression et d'ajuster son code.

Imaginons que dans un laboratoire de C nous développions maintenant une bataille navale dans le terminal. Tester de bout en bout de manière automatique un programme en C n'est pas une tâche évidente, surtout par manque d'outil adapté. Pour tester le fonctionnement global, il faut manuellement lancer une partie et jouer plusieurs coups pour vérifier le fonctionnement et vérifier à chaque étape si le jeu est cohérent et si l'affichage est correct. Une fois qu'une partie du jeu fonctionne, en développant le reste on risque de casser sans s'en rendre compte le reste.

Un dernier cas concret, en développant un petit shell en C++, pour tester l'implémentation des pipes, il faudra builder le shell ainsi que les CLIs accessibles, lancer le shell, puis lancer quelques commandes du type `echo hey there | toupper` voir si l'output est bien "HEY THERE", ce qui est très lent et pénible de devoir tester plein de cas limites (plusieurs pipes, symbole de pipe collé, exit du CLI à droite du pipe, ...)

En résumé, le manque de test automatisé ralentit le développement et l'apprentissage comme il n'y a pas de moyen de tout vérifier rapidement. Simplement ajouter des tests n'est pas suffisant, car les tests runner ne sont pas adaptés à des tests sur des exos, seulement du code de production, il manque une partie d'automatisation autour. De plus, le travail d'écriture de tests pour des tous petits exos où checker l'output suffirait largement.

## Expérience PLX
**Le défi est d'arriver à réduire la friction au strict minimum, d'automatiser toutes les étapes administratives et de fournir un feedback riche, automatique et rapide sur l'état d'avancement.**

Cette expérience consiste en:
1. **Zéro switch de fenêtre durant l'exo**  
PLX à gauche avec toute la consigne, l'IDE à droite dans un seul fichier utile, on tape un raccourci dans PLX pour que l'IDE ouvre le fichier de l'exo déjà créé sur le disque. La consigne s'affiche dans PLX, dès que le fichier ouvert est sauvé. Les erreurs de build précédentes sont toujours affichées, les résultats des tests précédents également.
1. **Comparaison avec solution**  
A défaut d'un feedback humain, pouvoir comparer sa réponse avec la solution d'un prof est déjà d'une grande aide. Une fois l'exercice résolu, si une solution est fournie, il sera possible de comparer sa réponse avec la solution affichée dans PLX.
1. **Retirer les étapes de build et d'exécution manuelles**  
Aucune connaissance du système de build ou de commandes de compilation n'est nécessaire, tout se fait automatiquement dès qu'un des fichiers est sauvé (il suffit donc de forcer avec Ctrl+S ou d'attendre que l'IDE sauve automatiquement)
1. **Retirer les étapes de comparaison d'output**  
l'output sera automatiquement comparé et une diff précise (avec highlighing des différences sur la ligne) sera affichée pour immédiatemment voir ce qui est différent ou si c'est pareil. La diff pourrait supporter un certain trimming de l'output ou des lignes afin de supporter certains espaces blancs insignifiants.
1. **Retirer toutes les rédactions manuelles de valeurs dans le terminal**  
Permettre de définir un contenu à injecter en `stdin`, y inclure plusieurs tests pour tester des variantes d'entrées et continuer le reste des assertions sur l'output.
1. **Transition fluide entre exos**  
Passer à l'exo suivant devrait prendre moins de 4 secondes, le temps d'un raccourci dans PLX pour afficher l'exo suivant et le temps que l'IDE réagisse à la demande d'ouverture du fichier en question.

## Contexte
Ce projet tire inspiration de [Rustlings](https://rustlings.cool/) qui permet de s'habituer aux erreurs du compilateur Rust en corrigeant des problèmes de compilation ou en complétant une centaine de petits exercices. Inspiré de Rustlings, d'autres languages ont suivis avec golings, cplings, ziglings, ... Ce même projet a inspirée [PRJS](https://github.com/samuelroland/prjs) (Practice Runner for JavaScript), développée à l'occasion du dernier labo libre de WEB et qui permet de s'entrainer sur des fonctions vérifiées via des tests unitaires écrits et lancés avec Vitest en arrière plan.

PLX pousse encore plus loin l'expérience en supportant plusieurs languages en y incluant la compilation automatique ainsi que des types de tests plus primitifs et simple à mettre en place qu'avec un framework de test comme Vitest en JS.

Note: contrairement à Rustlings, ce repository ne contient pas d'exercices réels, seulement le code du CLI PLX. Des exercices de démonstration seront écrits dans différents languages dans un sous dossier `examples`.

## Types de tests
- **Test runner**  
Supporter les tests runner existants les plus utilisés: GoogleTest et Junit, permet de simplement lancer ces tests runner en arrière plan, de générer un export des résultats et de les afficher dans PLX
- **Output simples**  
Comparer l'output console d'un exo soit avec un bout de texte définie, ou si la solution est définie, directement avec son output (permettant de facilement mettre à jour l'exo). Les tests pourront aussi inclure les arguments à passer au programme.
- **Output avec injection stdin**  
Pareil mais avec la possibilité d'injecter une string d'un coup ou par morceau (avec des délais d'attente) pour simuler des entrées utilisateurs.
- **External command output**  
Parfois l'output utile à comparer n'est pas celui du programme mais celui d'une commande externe. Par ex. si un exo doit créer des dossiers et des fichiers à des endroits spécifiques, le critère de vérification pourrait être que l'output de `tree` matche celui des dossiers générés par le code de la solution.

## Format texte
Afin de décrire les exos (titre, consigne, les tests associés de tous types), des fichiers textes seront utilisés pour être facilement versionnable et éditable. Soit on invente un format de texte de zéro, soit on définit juste un schéma dans un format existant (JSON, YAML, TOML, ...)

<!-- Voici un exemple de quelques idées de syntaxe pour se donner une idée -->
TODO: define this...

## Détails sur l'idée de projet pour PDG 2024
Techniquement, PLX sera une TUI (Text User Interface) en Rust (probablement en utilisant un framework de TUI comme ratatui) développée avec une équipe de gens motivés par apprendre un peu le Rust cet été. Elle sera donc crossplateform pour permettre un usage sous Linux, Windows et Mac.

**Fonctionnalités à implémenter durant PDG**
1. Format de fichiers à définir, affiner, demander des feedbacks à des profs, tester de convertir différents types d'exos pour trouver les limites.
1. Choisir un outil de build ([Xmake](https://xmake.io/#/) par ex.)
1. Implémenter le build et l'exécution de code C++, avec passage d'arguments
1. Implémentation la navigation dans les listes de compétences et exos, entrer dans les détails
1. Implémenter un mode watch solide pour lancer build+run automatiquement
1. Implémenter un affichage des détails d'un exo durant l'entrainement
1. Support des tests sur l'output simple
1. Support des tests runner
1. Support du C et du Java

**Exercices disponibles**  
Le recueil de [PRG1](https://github.com/PRG1-HEIGVD/PRG1_Recueil_Exercices) et celui de [PRG2](https://github.com/PRG1-HEIGVD/PRG1_Recueil_Exercices) sont des bonnes références sur laquel réfléchir à la conception, tenter de transformer dans un nouveau format, ...

**Conception**  
3 étudiants de PRG2 ont été interviewé et les problèmes exprimés plus haut se confirment. La vision générale a déjà été travaillée comme le montre ce README, mais il reste toute la conception détaillée pour définir à quoi va ressembler les listes d'exos et les détails et l'expérience de pratique d'un seul exo (comment afficher les erreurs de build et de crash, comment supporter les couleurs, comment faire des belles diff, comment permettre de changer de mise en page, ...).  
Et tout autant de considérations techniques: comment builder facilement sans config si possible du C, C++ et du Java, comment afficher l'exécution au fur et à mesure, quel outil de diffing choisir, comment implémenter un mode watch efficace, comment gérer les builds en arrière plan, comment récupérer le résultat de GoogleTest et autres, comment parser les fichiers textes décrivant les exos, comment produire des binaires pour les 3 OS...

**Equipe**  
Je souhaiterai travailler avec une équipe de 5 personnes motivées par cette vision de cette expérience PLX, si possible avec des bases de Rust ou l'envie d'apprendre/d'expérimenter cet été. Le but est aussi de faire des review de codes pour chaque PR et d'avoir des suite de tests, pour simplifier la maintenance à long terme et faciliter le développement bouillonnant sur 3 semaines.

## Usage à la HEIG-VD
En considérant le nombre important de cours qui propose déjà actuellement des exercices en C++ (PRG1, ASD, PCO), C (PRG2, SYE), Java (POO, DAI, POA), l'impact de cet outil pourrait être assez large si les enseignants sont motivés à l'adopter.  
Le but est de limiter au maximum la quantité de travail de restructuration des exercices existants et que les configurations de build sont non nécessaires ou très simple à mettre en place. Il sera facile aussi de motiver 2-3 étudiants pour aider à faire la migration pour faire un peu de travail répétitif manuel.

Si l'outil est utilisable (pas besoin d'être forcément fini) et déployable à la rentrée 2024 et que les exercices peuvent être facilement et rapidement adaptés (transformer les fichiers Markdown existants pour PRG1 par ex.), les étudiants de PRG1 pourront déjà en bénéficier ! Si GoogleTest est supporté, les exos de PCO pourront aussi être entrainé avec PLX. En SYE, les labos dans SO3 seraient grandement aidé (beaucoup d'étudiants perdent du temps à lancer les commandes de build, à éxecuter, à tester des commandes).

## Licence
Ce projet n'a pas encore de licence libre, il est donc techniquement propriétaire pour le moment, une fois le projet PDG terminé, le projet ne va pas s'arrêter, des finitions seront probablement nécessaires et la maintenance continuera sur notre temps libre si possible. Tout comme PRJS, le but est de demander à l'école de reprendre les droits d'auteurs afin de pouvoir publier ce projet sous licence libre. Une condition pour faire partie de l'équipe est donc d'être d'accord de cette finalité pour qu'au moment de faire les démarches administratives pour appliquer cette licence correctement tout se passe au plus simple.

