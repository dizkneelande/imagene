#!/bin/bash

# Define the font path as a variable
FONT_PATH="/usr/share/fonts/TTF/JetBrainsMono-Bold.ttf"

# Function to run a command and measure its execution time
run_benchmark() {
    echo "Running: $1"
    start=$(date +%s.%N)
    eval $1
    end=$(date +%s.%N)
    runtime=$(awk "BEGIN {print $end - $start}")
    echo "Execution time: $runtime seconds"
    echo ""
}

# Ensure the input file exists
if [ ! -f "jigglypuff.jpg" ]; then
    echo "Error: jigglypuff.jpg not found in the current directory."
    exit 1
fi

# Basic resizing
run_benchmark "./imagene jigglypuff.jpg resize:500,0 resized_jigglypuff.png"

# Applying a blur effect
run_benchmark "./imagene jigglypuff.jpg blur:5.0 blurred_jigglypuff.png"

# Adjusting brightness and contrast
run_benchmark "./imagene jigglypuff.jpg brightness:20 contrast:10 bright_contrast_jigglypuff.png"

# Rotating and flipping
run_benchmark "./imagene jigglypuff.jpg rotate:right flip:h rotated_flipped_jigglypuff.png"

# Adding a watermark 
run_benchmark "./imagene jigglypuff.jpg watermark:\"Pokemon\"\,\(0.5:0.5\)\,\(0.0:0.0:0.0:1.0\)\,\($FONT_PATH:0.5\) watermarked_jigglypuff.png"

# Cropping
run_benchmark "./imagene jigglypuff.jpg crop:100,100,300,300 cropped_jigglypuff.png"

# Inverting colors
run_benchmark "./imagene jigglypuff.jpg invert:true inverted_jigglypuff.png"

# Chaining multiple operations
run_benchmark "./imagene jigglypuff.jpg resize:400,0 blur:2.0 brightness:10 contrast:5 watermark:\"Jigglypuff\"\,\(0.5:0.9\)\,\(1.0:0.0:0.0:0.8\)\,\($FONT_PATH:0.1\) final_jigglypuff.png"

# Outputting to stdout (for piping to other programs)
run_benchmark "./imagene jigglypuff.jpg resize:200,0 stdout > small_jigglypuff.png"

# Complex pipeline for benchmarking
echo "Running complex pipeline for benchmarking"
start=$(date +%s.%N)
./imagene jigglypuff.jpg \
    resize:1000,0 \
    blur:3.0 \
    brightness:15 \
    contrast:10 \
    rotate:right \
    crop:50,50,900,900 \
    watermark:"Benchmark"\,\(0.5:0.1\)\,\(0.0:0.0:1.0:0.7\)\,\($FONT_PATH:0.1\) \
    flip:v \
    unsharpen:3.0,5 \
    invert:true \
    append:jigglypuff.jpg,right \
    resize:500,0 \
    watermark:"Final"\,\(0.1:0.9\)\,\(1.0:1.0:1.0:0.8\)\,\($FONT_PATH:0.05\) \
    benchmark_result.png
end=$(date +%s.%N)
runtime=$(awk "BEGIN {print $end - $start}")
echo "Complex pipeline execution time: $runtime seconds"