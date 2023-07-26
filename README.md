# sparse_matrix
A library to do solve sparse algebra problems.

## How is it meant to be used ?

You are meant to create build your matrix using a coo_mat that uses a HashMap to store the matrix. Then transform it to a csr_matrix that uses the Compressed Sparse Row format that will be used to solve your problem.
