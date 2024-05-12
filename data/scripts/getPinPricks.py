import cv2
import pandas as pd
import os
import numpy as np

# Define image directory
image_dir = "../screenshots/images/"

# Initialize dataframe to store results
results_df = pd.DataFrame(columns=["Image", "Coordinates"])

# Loop through each image in the directory
for filename in os.listdir(image_dir):
    if filename.endswith(".png"):
        # Load image
        img_path = os.path.join(image_dir, filename)
        img = cv2.imread(img_path, cv2.IMREAD_GRAYSCALE) 

        print(f"Processing {filename}...")

        # Threshold the image to create binary image (white dots on black)
        _, thresh = cv2.threshold(img, 1, 255, cv2.THRESH_BINARY)

        # Find contours (blobs/clumps of white pixels)
        contours, _ = cv2.findContours(thresh, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)

        # Filter contours to only keep those that are within the desired size range
        filtered_contours = [
            cnt
            for cnt in contours
            if 1 <= cv2.contourArea(cnt) <= 16
        ]

        # Get coordinates of the centroids of the filtered contours
        coordinates = [
            (int(cv2.moments(cnt)["m10"] / cv2.moments(cnt)["m00"]), int(cv2.moments(cnt)["m01"] / cv2.moments(cnt)["m00"])) 
            for cnt in filtered_contours
        ]

        # Add results to the dataframe
        results_df = pd.concat([results_df, pd.DataFrame({"Image": [filename], "Coordinates": [coordinates]})])

# Save the results to a csv file
results_df.to_csv("dot_coordinates.csv", index=False)

print("Dot coordinates saved to dot_coordinates.csv")
