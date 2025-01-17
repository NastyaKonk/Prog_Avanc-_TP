#Rapport TP programation avancé#

                         Partie 1 : 1 La bibliothèque image

Question 2 : 

	Le type DynamicImage permet de travailler avec une image dont le format peut varier notamment avec la couleurs,ou encore niveaux de gris.

	Et donc afin obtenir une image avec 3 canaux RGB représentés par des u8, on utilise .to_rgb8().


Question 3 : 

Si l'image de départ avait un canal alpha, la methode .to_rgb8(). aurait totalement ignoré le canal alpha, il aurait pas consequent totalement disparu.

De plus les pixels transparent seront opaque. 


Question 5 : 

L'image reste tout de même reconaissable même si les details ne sont plus du tout net.


                Partie 2 : Passage en monochrome par seuillage


Question 6 et 7 : 

Afin de récupérer la luminisité d'un pixel, nous avons utilisé la méthode to_luma() qui est fournie dans la bibliothéque image.
Cette methode permet de convertir un pixel de type Rgb <u8> en une valeur de luminosité qui est donc encapsulée dans un type Luma.


![alt text](./question6.png)

Donc la luminosité d’un pixel est calculée avec pixel.to_luma(), cette fonction renvoie une valeur pondérée des composantes RGB.

	•	La luminosité est ensuite utilisée pour décider si le pixel sera clair ou sombre, selon un seuil défini se seuil à été défini ici à >127.

Question 8 : 

