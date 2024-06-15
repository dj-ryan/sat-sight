# Sat-Sight

<p align="center"><img src="sat-sight-logo.png" width="30%"></img></p>

### A satellite star tracking suite of tools

> - Star database manipulation and projections
>    - Gnomonic projection
>    - (*planned*) Orthogonal projection
>    - (*planned*) Stereographic projection
>    - Quaternion Rotation
> - Image identification using "blur and prick" method
> - (*planned*) Star object follow for realtime tracking

 
## Blur and Prick method
This is a low computation high memory access operation. 

### Step 1 (Pre-Processing):

Start with a raw image of the stars taken by the sat:

![raw-image.png](/raw-image.png)
*image is a render from [sat-sight-view](https://github.com/dj-ryan/sat-sight-view)*

Increase the raw image contrast and calculate a gaussian blur over the image.

![blurred-image](/blurred-image.png)

### Step 2 (Orientation Identification):

With a known database of star positions "pinprick" the image with their locations. In other words, access the pixels that should contain a star if the sat was oriented in a particular direction. Take the values found and sum them to get a general "goodness" value for how well the image conforms to the star locations. The stars that are more in the correct position will have a higher value as they are more near the center of the blurred star. The stars further from the correct position will have a lower value.

This can be done in one of two ways:
1. With a predefined list of pixel coordinate values and a list of their respective orientation vectors we pinprick the original blurred image with these values until we find a orientation vector that has the highest match. Depending on the size and resolution (the distance between the vectors) of our database we can get an approximant orientation.
- **Pros:** Extremely fast memory intensive operation. 
- **Cons:** Only an approximation based

2. With a database of star galactic longitude and latitude points, pick a random orientation and project these points onto a 2D plane. This would be the supposed star locations that the sat would see if if it was oriented in this way. Use these pixel values to pinprick the original blurred image as before. Loop trough orientations to find the one with the highest goodness value. 
- **Pros:** Dynamic ability to generate star positions
- **Cons:** Compute intensive

### Step 3 (Tracking):

Once a vector has been established a tracking algo can be used to keep a lock on the orientation *(planned)*

### Summery:
![sat-funny](https://www.jpl.nasa.gov/nmp/st6/IMAGES/starsearch3.jpg)






