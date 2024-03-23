import pandas as pd
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import plotly.express as px

# **1. Load the CSV data**
data = pd.read_csv("star_formated_raw.csv")  # Replace 'your_star_data.csv' with your filename

# **2. Extract galactic coordinates**
galactic_lat = data["GLAT"]
galactic_lon = data["GLON"]

# **3. Matplotlib 3D Scatter Plot**
fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')

ax.scatter(galactic_lon, galactic_lat, color='blue', marker='.')
ax.set_xlabel('Galactic Longitude')
ax.set_ylabel('Galactic Latitude')
ax.set_zlabel('Z')  # Add a Z label if you have a third dimension
plt.title('Star Locations (Matplotlib)')
plt.show()

# **4. Plotly 3D Scatter Plot**
fig = px.scatter_3d(data, x='galactic_lon', y='galactic_lat', color='blue', 
                    title='Star Locations (Plotly)')
fig.show()
