NPY Streams
===========

Numpy arrays are an efficient, commonly used data format in python.
A good starting point for machine learning is to have your features in a numpy array, with rows being examples and columns being features.

If your feature engineering process is expensive, you might want to run this first and save your engineered features to disc an a `.npy` file.
Unfortunately, if the number of rows and columns is large, this may involve storing a very large number of engineered features, which are then written to disc at the end of the feature engineering step.

CSV files offer an obvious way around this, since they can be written line-by-line.
However, they are slow to read into python, and take up for more disc space.

The solution we introduce is to write `.npy` files in a _streaming_ fashion.
We provide a struct `NPYStream`, whose `write` method takes a `Vec<f32>` and promptly writes this to disc (subject to delays introduced by the buffered writer).
This allows complex feature engineering without having to worry about the memory consumed by storing the results.
