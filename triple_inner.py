import pymeshlab
import numpy as np
import sys

suffix = sys.argv[1]
targetfacenum = int(sys.argv[2])

# Create a new MeshSet
ms = pymeshlab.MeshSet()

# Load the STL file
ms.load_new_mesh(f'out/inner{suffix}.ply')

ms.meshing_decimation_quadric_edge_collapse(
    targetfacenum=targetfacenum,
    qualitythr=0.3,
    planarquadric=True,
    preserveboundary=True,
    preservenormal=True,
    optimalplacement=True
)

# First copy (just rotated)
ms.set_matrix(transformmatrix=np.array([
    [-1, 0, 0, 0],
    [0, -1, 0, 0],
    [0, 0, 1, 0],
    [0, 0, 0, 1]
]), freeze=True)

# Create second copy, z-shift
ms.generate_copy_of_current_mesh()
ms.set_matrix(transformmatrix=np.array([
    [1, 0, 0, 0],
    [0, 1, 0, 0],
    [0, 0, 1, 58],
    [0, 0, 0, 1]
]))

ms.set_current_mesh(0)

# Create third copy, z-shift
ms.generate_copy_of_current_mesh()
ms.set_matrix(transformmatrix=np.array([
    [1, 0, 0, 0],
    [0, 1, 0, 0],
    [0, 0, 1, 113],
    [0, 0, 0, 1]
]))

# Merge all meshes
ms.generate_boolean_union(first_mesh=0, second_mesh=1)
ms.generate_boolean_union(first_mesh=3, second_mesh=2)

# Save the result
ms.save_current_mesh(f'out/inner{suffix}_many.stl')
