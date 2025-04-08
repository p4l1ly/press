import pymeshlab
import numpy as np
import sys

name = sys.argv[1]

# Create a new MeshSet
ms = pymeshlab.MeshSet()

# Load the STL file
ms.load_new_mesh(f'out/{name}.ply')

print("number of faces before:", ms.current_mesh().face_number())

targetfacenum = int(input("Enter target face number: "))

ms.meshing_decimation_quadric_edge_collapse(
    targetfacenum=targetfacenum,
    qualitythr=0.3,
    planarquadric=True,
    preserveboundary=True,
    preservenormal=True,
    optimalplacement=True
)

ms.save_current_mesh(f'out/{name}_simp.stl')
