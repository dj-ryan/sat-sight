import pandas as pd
import plotly.graph_objects as go
import numpy as np


# **1. Load the CSV data**
data = pd.read_csv("star_formated_raw.csv")  # Replace 'your_star_data.csv' with your filename

# **2. Extract galactic coordinates**
galactic_lat = data["GLAT"]
galactic_lon = data["GLON"]
#star_brightness = data["Vmag"]

# Coordinate Transformation (degrees to radians, then to Cartesian)
theta = np.radians(galactic_lat)
phi = np.radians(galactic_lon)  # Adjust for latitude convention

radius = 10 
x = radius * np.cos(theta) * np.cos(phi)
y = radius * np.cos(theta) * np.sin(phi)
z = radius * np.sin(theta)

# Create the Plotly figure
fig = go.Figure()

# Add star locations
fig.add_trace(go.Scatter3d(
    x=x, y=y, z=z,
    mode='markers',
    marker=dict(size=3, color=data["Vmag"]+5)
))

# Customize the sphere layout
fig.update_layout(scene = dict(
                    xaxis_title='X',
                    yaxis_title='Y',
                    zaxis_title='Z'),
                  margin=dict(l=0, r=0, b=0, t=0),
                  )

# Display the plot
fig.show()
