"use client"

import { useRef } from "react"
import { Canvas, useFrame } from "@react-three/fiber"
import { PresentationControls, ContactShadows, Environment } from "@react-three/drei"
import * as THREE from "three"

function KeyboardModel() {
  const groupRef = useRef<THREE.Group>(null)

  useFrame((state) => {
    if (groupRef.current) {
      groupRef.current.rotation.y = Math.sin(state.clock.getElapsedTime() * 0.3) * 0.15
    }
  })

  const keyRows = 5
  const keyCols = 12
  const keyWidth = 0.6
  const keyHeight = 0.6
  const keyDepth = 0.3
  const gap = 0.1
  const baseWidth = keyCols * (keyWidth + gap) + gap
  const baseHeight = keyRows * (keyHeight + gap) + gap
  const baseDepth = 0.2

  const keys = []
  for (let row = 0; row < keyRows; row++) {
    for (let col = 0; col < keyCols; col++) {
      const x = col * (keyWidth + gap) - baseWidth / 2 + keyWidth / 2
      const z = row * (keyHeight + gap) - baseHeight / 2 + keyHeight / 2
      keys.push({ x, z, key: `${row}-${col}` })
    }
  }

  return (
    <group ref={groupRef}>
      <mesh position={[0, -baseDepth / 2, 0]}>
        <boxGeometry args={[baseWidth, baseDepth, baseHeight]} />
        <meshStandardMaterial color="#1a1a2e" roughness={0.8} />
      </mesh>

      {keys.map(({ x, z, key }) => (
        <mesh key={key} position={[x, 0, z]}>
          <boxGeometry args={[keyWidth, keyDepth, keyHeight]} />
          <meshStandardMaterial color="#7ebab5" roughness={0.4} metalness={0.1} />
        </mesh>
      ))}
    </group>
  )
}

export function HeroKeyboard() {
  return (
    <div className="h-[400px] w-full">
      <Canvas camera={{ position: [0, 4, 6], fov: 45 }} dpr={[1, 2]}>
        <color attach="background" args={["#090a0c"]} />
        <PresentationControls
          global
          rotation={[0.1, 0.1, 0]}
          polar={[-0.5, 0.5]}
          azimuth={[-1, 1]}
        >
          <KeyboardModel />
        </PresentationControls>
        <ContactShadows
          position={[0, -0.5, 0]}
          opacity={0.5}
          scale={10}
          blur={2}
          far={4}
        />
        <Environment preset="city" />
        <ambientLight intensity={0.3} />
        <spotLight position={[5, 5, 5]} angle={0.5} penumbra={1} intensity={1} castShadow />
      </Canvas>
    </div>
  )
}
