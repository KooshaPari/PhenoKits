package com.phenotype.eyetracker

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.drawBehind
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.unit.*
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.launch

// FFI imports (would be generated from Rust crate)
// import eyetracker_ffi.*

data class GazeData(
    val x: Float,
    val y: Float,
    val confidence: Float,
    val timestamp: Long
)

class EyetrackerViewModel : ViewModel() {
    private val _gazeState = mutableStateOf<GazeData?>(null)
    val gazeState: State<GazeData?> = _gazeState

    private val _isTracking = mutableStateOf(false)
    val isTracking: State<Boolean> = _isTracking

    fun startTracking() {
        _isTracking.value = true
        viewModelScope.launch {
            // Initialize Rust estimator via JNI
            // val estimator = GazeEstimatorFFI(camera, screen)
        }
    }

    fun stopTracking() {
        _isTracking.value = false
    }

    fun updateGaze(x: Float, y: Float, confidence: Float) {
        _gazeState.value = GazeData(x, y, confidence, System.currentTimeMillis())
    }
}

@Composable
fun EyetrackerScreen(viewModel: EyetrackerViewModel) {
    val gazeData = viewModel.gazeState.value
    val isTracking = viewModel.isTracking.value

    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(Color.Black)
    ) {
        // Camera preview would go here
        CameraPreviewPlaceholder()

        // Gaze cursor overlay
        if (isTracking && gazeData != null) {
            GazeCursor(
                x = gazeData.x,
                y = gazeData.y,
                confidence = gazeData.confidence
            )
        }

        // Info panel
        Column(
            modifier = Modifier
                .align(Alignment.TopStart)
                .padding(16.dp)
                .background(Color.Black.copy(alpha = 0.6f), shape = androidx.compose.foundation.shape.RoundedCornerShape(8.dp))
                .padding(12.dp)
        ) {
            Text(
                "Eye Tracker MVP",
                style = TextStyle(color = Color.White, fontSize = 16.sp),
                modifier = Modifier.padding(bottom = 8.dp)
            )

            if (isTracking && gazeData != null) {
                Text(
                    "Gaze: (${gazeData.x.toInt()}, ${gazeData.y.toInt()})",
                    style = TextStyle(color = Color.Gray, fontSize = 12.sp)
                )
                Text(
                    "Confidence: ${String.format("%.1f%%", gazeData.confidence * 100)}",
                    style = TextStyle(color = Color.Gray, fontSize = 12.sp)
                )
            } else {
                Text(
                    "Initializing...",
                    style = TextStyle(color = Color.Gray, fontSize = 12.sp)
                )
            }
        }

        // Control button
        Button(
            onClick = {
                if (isTracking) viewModel.stopTracking() else viewModel.startTracking()
            },
            modifier = Modifier
                .align(Alignment.BottomCenter)
                .padding(16.dp),
            colors = androidx.compose.material3.ButtonDefaults.buttonColors(
                containerColor = if (isTracking) Color.Red else Color.Green
            )
        ) {
            Text(if (isTracking) "Stop Tracking" else "Start Tracking")
        }
    }
}

@Composable
fun GazeCursor(x: Float, y: Float, confidence: Float) {
    val density = LocalDensity.current
    Box(
        modifier = Modifier
            .size(20.dp)
            .offset(
                x = with(density) { x.toDp() - 10.dp },
                y = with(density) { y.toDp() - 10.dp }
            )
            .drawBehind {
                drawCircle(
                    color = Color.Red,
                    radius = 10f,
                    style = Stroke(width = 2f),
                    alpha = if (confidence > 0.5f) 0.8f else 0.3f
                )
            }
    )
}

@Composable
fun CameraPreviewPlaceholder() {
    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(Color.DarkGray),
        contentAlignment = Alignment.Center
    ) {
        Text(
            "Camera Preview\n(CameraX + MLKit FaceMesh)",
            color = Color.LightGray,
            textAlign = androidx.compose.ui.text.style.TextAlign.Center
        )
    }
}

@Composable
fun Button(
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    colors: androidx.compose.material3.ButtonColors = androidx.compose.material3.ButtonDefaults.buttonColors(),
    content: @Composable () -> Unit
) {
    androidx.compose.material3.Button(onClick = onClick, modifier = modifier, colors = colors) {
        content()
    }
}
