**Anastasiia Kononenko** <br>
**Hajar El Kasmi**	<br>
**Groupe : 33A**		     
				 
# RAPPORT TP PROGRAMMATION AVANCÉ



### Partie 1 : La bibliothèque image

**Question 2 :** 

	Le type DynamicImage permet de travailler avec une image dont le format peut varier notamment avec la couleur,ou encore les niveaux de gris.

	Et donc afin d'obtenir une image avec 3 canaux RGB représentés par des u8, on utilise .to_rgb8().


**Question 3 :** 

Si l'image de départ avait un canal alpha, la methode .to_rgb8(). aurait totalement ignoré le canal alpha, il aurait par consequent totalement disparu.

De plus les pixels transparents seront opaques. 


**Question 4 :** 

Pour cela on utilise la fonction get_pixel comme ci-dessous : 

![alt text](./Question4.png)


**Question 5 :** 

L'image reste tout de même reconnaissable, même si les détails ne sont plus du tout nets.


### Partie 2 : Passage en monochrome par seuillage


**Question 6 et 7 :** 

Afin de récupérer la luminosité d'un pixel, nous avons utilisé la méthode to_luma() qui est fournie dans la bibliothèque image.
Cette méthode permet de convertir un pixel de type Rgb <u8> en une valeur de luminosité qui est donc encapsulée dans un type Luma.


![alt text](./question6.png)

Donc, la luminosité d’un pixel est calculée avec pixel.to_luma(), cette fonction renvoie une valeur pondérée des composantes RGB.

	•	La luminosité est ensuite utilisée pour décider si le pixel sera clair ou sombre, selon un seuil défini ce seuil à été défini ici à >127.

**Question 8 :** 

La mode  seuil ne comprenait pas d'option qui permettait à l'utilisateur de spécifier un type de couleur, foncé ou clair, ou de changer l'image en noir ou en blanc. 
Nous avons donc créer ses options : 


![alt text](./question8.png)

Dans une variable default, nous avons mis du blanc et du noir comme demandé dans la question, puis nous avons étendu cela à tous les panels de couleur grâce au parse_color.



```
$ cargo run -- image.jpg output.png seuil --dark-color noir --light-color blanc

```

Via la ligne de commande il suffit de spécifier 

```-dark-color" ou "-light-color" (dark oou light) ```

La commande pour mettre l'image en noir et blanc : 

``` $ cargo run -- image.jpg output.png seuil --dark-color noir --light-color blanc ```

La commande pour mettre d'autres nuances de couleur

``` $ cargo run -- image.jpg output.png seuil --dark-color jaune --light-color vert ```



### PARTIE 3 : 

**Question 9 - 10 :** 

Afin de calculer la distance entre deux couleurs, nous utilisons la fonction color_distance qui permet de calculer la distance entre deux couleurs. 
La méthode de calucle utiliser est donc la distance euclidienne appliquée à notre espace RGB.

Ici on calcule séparément la différence des composantes RGB :

![alt text](./question9-2.png)

Une fois les différences caclulées nous les mettons au carré et nous les additionnons à l'aide de cette formule : <br>

```dr * dr + dg * dg + db * db ``` <br>
On fait ensuite la racine carrée de tout cela grâce au <br> ``` dr * dr + dg * dg + db * db).sqrt ```


**Question 11 :** 

Dans la variable default une variable numérique qui est 8, celle-ci représente le nombre de couleurs qui constituera notre palette.
Donc, on a choisi d'utiliser toutes les couleurs de notre palette pour ne pas avoir d'erreur.


<img src="./Question11-1.png" alt="drawing" width="200"/>

![alt text](./question11.png)


### PARTIE 4 : Dethering

**Question 12 :** 

![alt text](./Question12.png)

Dans un tramage aléatoire, pour chaque pixel de l’image, on tire au hasard un seuil entre 0 et 1, et on met le pixel en blanc si sa luminosité est supérieure au seuil.

### PARTIE 7 : La bibliothèque argh

**Question 21 :** 

Voici ce que l'on obtient avec la commande '$ cargo run -- --help'
![alt text](./Question21-1.png)

Cependant pour afficher les arguments il faut effectuer la commande :

'$ cargo run -- seuil --help'
![alt text](./Question21.png)



**Question 22 :** 

Voici comment nous avons determiné le type Rust correspondant à une sélection d’options fournies par l’utilisateur.

![alt text](./Question22.png)

**Question 23 :**

On a implémenté dans le main  cette partie : 

"let args: DitherArgs = argh::from_env();"

qui sert à analyser et à récupérer les arguments passés par l’utilisateur à travers la ligne de commande.
Et on retrouve l'argument args initialisé dans le mode comme vous pouvez le voir ci-dessous. 

![alt text](./Question23.png)

