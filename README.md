# plx
> **P**ractice programming exos in a delightful **L**earning e**X**perience

**STATUT: conception pour le projet PDG en HES d'été**

## Pourquoi
Pour expérimenter un cours de programmation conçu autour de la pratique délibérée, les exercices sont au coeur de l'apprentissage, cependant avoir des exercices avec petits programmes ou fonctions à implémenter ne garantit pas que l'expérience de pratique sera efficiente. Selon la pratique délibérée, l'apprentissage profond passe par une boucle de feedback la plus courte possible.

Prenons l'exemple d'un exercice de C fourni dans un PDF, incluant une consigne, un bout de code de départ et un output attendu, ainsi qu'une solution sur la page suivante.  
Pour résoudre l'exercice, une fois la consigne lue, nous allons créer un fichier `main.c` manuellement, copier-coller le code de départ de la consigne, lire le code existant et compléter les parties à développer. Une fois terminé, passons à la compilation en ouvrant un terminal dans l'IDE en tapant `gcc main main.cpp & main`, euh zut c'était .`gcc -o main main.cpp && ./main`, puis comparons l'output manuellement pour voir si c'est bien le résultat attendu, réouvrons la consigne et non il manque une partie! Revenons au code, on corrige le soucis et on relance le build et l'exécution. Est-ce que l'output est bon cette fois ? La fin a l'air mieux mais est-ce que ces 20 lignes sont vraiment toutes correctes ? On va dire que c'est bon... Vérifions maintenant notre code avec la solution. Okay, on aurait pu utiliser `printf` au lieu de 2 fois `puts()` pour afficher le nom complet. Passons à l'exo suivant, cherchons sa consigne, la voilà,...

Tous ces petits étapes supplémentaires autour de la rédaction du code semblent insignifiantes à première vue mais leur cumul résulte en une friction générale importante. En plus, il y aura beaucoup moins d'exécutions manuels qu'avec des tests automatisé, c'est à dire beaucoup moins d'occasions de connaître la progression et d'ajuster son code.

Imaginons que dans un laboratoire de C nous développions maintenant une bataille navale dans le terminal. Tester de bout en bout de manière automatique un programme en C n'est pas une tâche évidente, surtout par manque d'outil adapté. Pour tester le fonctionnement global, il faut manuellement lancer une partie et jouer plusieurs coups pour vérifier le fonctionnement et vérifier à chaque étape si le jeu est cohérent et si l'affichage est correct. Une fois qu'une partie du jeu fonctionne, en développant le reste on risque de casser sans s'en rendre compte le reste.

En résumé, le manque de test automatisé ralentit le développement et l'apprentissage comme il n'y a pas de moyen de tout vérifier rapidement. Simplement ajouter des tests n'est pas suffisant, car les tests runner ne sont pas adaptés à des tests sur des exos, seulement du code de production, il manque une partie d'automatisation autour.

## Expérience PLX
**Le défi maintenant est d'arriver à réduire la friction au strict minimum, d'automatiser toutes les étapes administratives et de fournir un feedback riche et rapide sur l'état d'avancement.**

Cette expérience 

Le but est de mettre en place une expérience de pratique avec une boucle de feedback la plus courte possible.

<!-- ## Usage à la HEIG-VD -->


## Contexte
Ce projet tire inspiration de [Rustlings](https://rustlings.cool/) qui permet d'apprendre de s'habituer aux erreurs du compilateur Rust en corrigeant des problèmes de compilation ou en complétant des petits exercices. Ce même projet a inspirée [PRJS](https://github.com/samuelroland/prjs) (Practice Runner for JavaScript) et qui permet de s'entrainer des fonctions en JavaScript vérifiées via des tests unitaires écrits et lancés avec Vitest en arrière plan. PRJS a été développé pour le dernier labo libre de WEB.

PLX pousse encore plus loin l'expérience en supportant plusieurs languages compilés (voir après)

Note: contrairement à Rustlings, ce repository ne contient pas d'exercices réels, seulement le code du CLI PLX. Des exercices de démonstration seront écrits dans différents languages dans un sous dossier `examples`.



