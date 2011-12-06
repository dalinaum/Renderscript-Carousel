/**
 * carousel.rs
 * Copyright (c) 2011 daoki2
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

#pragma version(1)
#pragma rs java_package_name(com.android.example.carousel)

#include "rs_graphics.rsh"

#define NUM_ITEMS 10
#define RADIUS 828

rs_program_vertex gProgVertex;
rs_program_fragment gProgFragmentTexture;
rs_program_store gProgStoreBlendNone;
rs_program_raster gCullNone;
rs_sampler gLinearClamp;

rs_allocation gTex_00, gTex_01, gTex_02, gTex_03, gTex_04;
rs_allocation gTex_05, gTex_06, gTex_07, gTex_08, gTex_09;

typedef struct __attribute__((packed, aligned(4))) Bitmaps {
    rs_allocation data;
} Bitmaps_t;
Bitmaps_t *bitmap;

float gPx = 0;
float gPy = 0;
float gVx = 0;
float gVy = 0;

//float gDt = 0;

static float vertices[30];
static float len;
static int rot = 360/NUM_ITEMS/2; // Default angle
static int initialized = 0;

void init() {
}

static void displayCarousel() {    
    // Default vertex shader
    rsgBindProgramVertex(gProgVertex);

    // Setup the projection matrix
    rs_matrix4x4 proj;    
    float aspect = (float)rsgGetWidth() / (float)rsgGetHeight();
    rsMatrixLoadPerspective(&proj, 30.0f, aspect, 0.1f, 1500.0f);    
    rsgProgramVertexLoadProjectionMatrix(&proj);
    
    // Fragment shader with texture
    rsgBindProgramStore(gProgStoreBlendNone);
    rsgBindProgramFragment(gProgFragmentTexture);
	rsgBindProgramRaster(gCullNone);
	rsgBindSampler(gProgFragmentTexture, 0, gLinearClamp);

    // Reduce the rotation speed
	if (gVx != 0) {
		rot = rot + gVx;
		gVx = gVx * 0.95;
		if (fabs(gVx) < 0.1) {
			gVx = 0;
		}
	}
    
    // Load vertex matrix as model matrix
    rs_matrix4x4 matrix;
    rsMatrixLoadTranslate(&matrix, 0.0f, 0.0f, -400.0f); // camera position
    rsMatrixRotate(&matrix, rot, 0.0f, 1.0f, 0.0f); // camera rotation
    rsgProgramVertexLoadModelMatrix(&matrix);
	
	// Draw the rectangles
    Bitmaps_t *b = bitmap;
    for (int i = 0; i < 10; i++) {
	    rsgBindTexture(gProgFragmentTexture, 0, b->data);
    	rsgDrawQuadTexCoords(
    		vertices[i*3],
    		-(len/2),
    		vertices[i*3+2],
    		0,1,
    		vertices[i*3],
    		len/2,
    		vertices[i*3+2],
    		0,0,
    		vertices[i == 9 ? 0 : (i+1)*3],
    		len/2,
    		vertices[i == 9 ? 0 + 2 : (i+1)*3 + 2],
    		1,0,
    		vertices[i == 9 ? 0 : (i+1)*3],
    		-(len/2),
    		vertices[i == 9 ? 0 + 2 : (i+1)*3 + 2],
    		1,1
    	);
	    b++;
    }
}

static void initBitmaps() {
    // Set the bitmap address to the structure
	Bitmaps_t *b = bitmap;
	b->data = gTex_00; b++;
	b->data = gTex_01; b++;
	b->data = gTex_02; b++;
	b->data = gTex_03; b++;
	b->data = gTex_04; b++;
	b->data = gTex_05; b++;
	b->data = gTex_06; b++;
	b->data = gTex_07; b++;
	b->data = gTex_08; b++;
	b->data = gTex_09; b++;

    // Calculate the length of the polygon
	len = RADIUS * 2 * sin(M_PI/NUM_ITEMS);
	
	// Calculate the vertices of rectangles
	float angle;
	for (int i = 0; i < NUM_ITEMS; i++) {
		angle = i * 360 / NUM_ITEMS;
		vertices[i*3] = sin(angle * M_PI / 180) * RADIUS;
		vertices[i*3 + 1] = 0;
		vertices[i*3 + 2] = -cos(angle * M_PI / 180) * RADIUS;
	}
}

int root() {
	if (initialized == 0) {
		initBitmaps();
		initialized = 1;
	}
	
    //gDt = rsGetDt();
    rsgClearColor(0.0f, 0.0f, 0.0f, 0.0f);
    rsgClearDepth(1.0f);

    displayCarousel();
    
    return 10;
}
