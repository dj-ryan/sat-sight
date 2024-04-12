import pandas as pd
from math import radians, sin, cos, asin, sqrt

def get_nearest_neighbors(df, k, csv_file="data.csv"):
  """
  This function takes a CSV file, DataFrame, and number of neighbors (k) as input. 
  Returns a DataFrame with k nearest neighbors for each point.

  Args:
      df: Optionally, a pre-loaded DataFrame.
      k: Number of nearest neighbors to find.
      csv_file: The path to your CSV file (default is 'data.csv').
  """
  if df is None:
      df = pd.read_csv(csv_file)  # Load data from CSV

  # Convert degrees to radians
  df['lat_rad'] = df['GLAT'].apply(radians)
  df['lon_rad'] = df['GLON'].apply(radians)

  

  def distance(lat1, lon1, lat2, lon2):
    """
    This function calculates the distance between two points on a sphere using the Haversine formula.
    """
    R = 6371  # Earth's radius (in kilometers)

    lat1, lon1, lat2, lon2 = map(radians, [lat1, lon1, lat2, lon2])

    dlon = lon2 - lon1
    dlat = lat2 - lat1

    a = sin(dlat/2)**2 + cos(lat1) * cos(lat2) * sin(dlon/2)**2
    c = 2 * asin(sqrt(a))
    return R * c

  nearest_neighbors = []
  for i in range(len(df)):
    print(f"Calculating nearest neighbors for point {i+1}/{len(df)}")
    point = df.iloc[i]
    distances = df.apply(lambda row: distance(point['lat_rad'], point['lon_rad'], row['lat_rad'], row['lon_rad']), axis=1)
    distances.iloc[i] = float('inf')
    nearest_neighbors.append(distances.sort_values(ascending=True).head(k + 1).iloc[1:])

  # Add nearest neighbor columns to the DataFrame
  for i, neighbors in enumerate(nearest_neighbors):
    for j, neighbor in enumerate(neighbors.index):
      df.loc[i, f'n{j+1}_lat'] = df.loc[neighbor, 'lat']
      df.loc[i, f'n{j+1}_lon'] = df.loc[neighbor, 'lon']

    # Print out the nearest neighbor columns in a formatted way
    for i in range(1, k+1):
        print(f"Nearest Neighbor {i}:")
        print(df[f"n{i}_lat"])
        print(df[f"n{i}_lon"])
        print()

  # Drop temporary columns
  df.drop(columns=['lat_rad', 'lon_rad'], inplace=True)

  return df

# Example usage with a CSV file
k = 5
df = get_nearest_neighbors(None, k, csv_file="star_formated_raw.csv")  

print(df.to_string()) 
