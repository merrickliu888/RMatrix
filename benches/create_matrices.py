import numpy as np
import json

seed = 42
np.random.seed(seed)

def generate_matrix_set(n_start: int, n_end: int, step_size: int = 1) -> list[np.ndarray]:
    """
    Generate a set of random matrices of size n x n where n_start <= n <= n_end
    """
    return [np.random.rand(n, n) for n in range(n_start, n_end + 1, step_size)]

def save_matrices(matrices: list[np.ndarray], filename: str):
    """
    Save a list of matrices to a json file
    """
    serialized_matrices = [matrix.tolist() for matrix in matrices]
    with open(f"./benches/{filename}", 'w') as f:  
        json.dump(serialized_matrices, f)

if __name__ == "__main__":
    matrices = generate_matrix_set(10, 1000, 10)
    save_matrices(matrices, "matrices.json")
