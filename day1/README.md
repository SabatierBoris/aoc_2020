# Jour 1

## Partie 1
J'utilise un Set pour se souvenir de tous les nombres déjà lu.
Je calcule la différence attendue, et si elle est présente dans le Set : Gagné !

## Partie 2
J'ai toujours un Set pour l'ensemble des nombres déjà parcouru.
A chaque nouveau nombre, je parcourt le Set et je calcule la somme et le produit.
Et j'ajoute ça dans une HashMap avec comme clé la somme et valeur le produit.

Bien sur, avant ça, je regarde si la différence attendue est une clé du la HashMap.
Si c'est le cas, j'utilise la valeur pour faire le produit final.
