import('./pkg')
    .then((wasm) => {
        const { RustyRays } = wasm

        // Get canvas and context
        const canvas = document.getElementById('canvas')
        const ctx = canvas.getContext('2d')

        // Scene description - this will be sent to Rust
        const scene = {
            camera: {
                position: [-2, 2, 1],
                target: [0, 0, -1],
                fov: 90,
                samples_per_pixel: 24,
                max_depth: 10,
                image_width: canvas.width,
                aspect_ratio: canvas.width / canvas.height,
            },
            objects: [
                {
                    type: 'sphere',
                    center: [0.0, -100.5, -1.0],
                    radius: 100.0,
                    material: {
                        type: 'lambertian',
                        albedo: [0.8, 0.8, 0.0],
                    },
                },
                {
                    type: 'sphere',
                    center: [0.0, 0.0, -1.2],
                    radius: 0.5,
                    material: {
                        type: 'lambertian',
                        albedo: [0.1, 0.2, 0.5],
                    },
                },
                {
                    type: 'sphere',
                    center: [-1.0, 0.0, -1.0],
                    radius: 0.5,
                    material: {
                        type: 'dielectric',
                        ir: 1.5,
                    },
                },
                {
                    type: 'sphere',
                    center: [-1.0, 0.0, -1.0],
                    radius: 0.4,
                    material: {
                        type: 'dielectric',
                        ir: 0.6666666667,
                    },
                },
                {
                    type: 'sphere',
                    center: [1.0, 0.0, -1.0],
                    radius: 0.5,
                    material: {
                        type: 'metal',
                        albedo: [0.8, 0.6, 0.2],
                        fuzz: 1.0,
                    },
                },
            ],
        }

        // Scene manipulation functions
        const sceneManager = {
            // addSphere(center, radius, material) {
            //     const sphere = {
            //         type: 'sphere',
            //         center: [...center],
            //         radius,
            //         material: { ...material },
            //     }
            //     scene.objects.push(sphere)
            //     return scene.objects.length - 1 // Return ID
            // },
            //
            // removeObject(id) {
            //     if (id >= 0 && id < scene.objects.length) {
            //         scene.objects.splice(id, 1)
            //     }
            // },
            //
            // moveObject(id, position) {
            //     if (id >= 0 && id < scene.objects.length) {
            //         scene.objects[id].center = [...position]
            //     }
            // },
            //
            // updateMaterial(id, material) {
            //     if (id >= 0 && id < scene.objects.length) {
            //         scene.objects[id].material = { ...material }
            //     }
            // },
            //
            // setCamera(position, target, fov, width, aspectRatio) {
            //     scene.camera.position = [...position]
            //     scene.camera.target = [...target]
            //     if (fov !== undefined) scene.camera.fov = fov
            //     if (width !== undefined) raytracer.image_width = width
            //     if (aspectRatio !== undefined) scene.camera.aspect_ratio = aspectRatio
            // },
            //
            // setCameraSize(width, height) {
            //     raytracer.image_width = width
            //     scene.camera.aspect_ratio = width / height
            // },

            getSceneJson() {
                return JSON.stringify(scene, null, 2)
            },

            loadScene(jsonString) {
                try {
                    const newScene = JSON.parse(jsonString)
                    Object.assign(scene, newScene)
                    return true
                } catch (e) {
                    console.error('Failed to load scene:', e)
                    return false
                }
            },
        }

        // Create RustyRays instance with scene JSON
        const raytracer = RustyRays.new(sceneManager.getSceneJson())

        // Create image data buffer
        const imageData = ctx.createImageData(raytracer.image_width, raytracer.image_height)
        const data = imageData.data
        const data32 = new Uint32Array(imageData.data.buffer)

        // Create shuffled list of all pixel indices
        const totalPixels = raytracer.image_width * raytracer.image_height
        const pixelOrder = Array.from({ length: totalPixels }, (_, i) => i)

        // Shuffle the array using Fisher-Yates algorithm
        for (let i = pixelOrder.length - 1; i > 0; i--) {
            const j = Math.floor(Math.random() * (i + 1))
            ;[pixelOrder[i], pixelOrder[j]] = [pixelOrder[j], pixelOrder[i]]
        }

        // Progressive rendering function
        let currentBatch = 0
        const batchSize = 50 // Render 50 pixels per frame

        function renderNextBatch() {
            const startIdx = currentBatch * batchSize
            const endIdx = Math.min(startIdx + batchSize, totalPixels)

            for (let i = startIdx; i < endIdx; i++) {
                const pixelIndex = pixelOrder[i]
                const x = pixelIndex % raytracer.image_width
                const y = Math.floor(pixelIndex / raytracer.image_width)
                // Get color from raytracer (returns packed 0xFFBBGGRR u32)
                const packedColor = raytracer.render_pixel(x, y)
                data32[pixelIndex] = packedColor
            }

            // Update the canvas with current progress
            ctx.putImageData(imageData, 0, 0)

            currentBatch++

            // Continue rendering if there are more pixels
            if (startIdx < totalPixels) {
                requestAnimationFrame(renderNextBatch)
            } else {
                console.log('Rendering complete!')
            }
        }

        // Expose sceneManager globally for console debugging
        window.sceneManager = sceneManager
        window.scene = scene

        // Add some basic controls
        const controls = document.createElement('div')
        controls.style.cssText =
            'position: fixed; top: 10px; left: 10px; background: rgba(0,0,0,0.8); color: white; padding: 10px; font-family: monospace; border-radius: 5px; z-index: 1000;'
        controls.innerHTML = `
            <div>Scene Controls</div>
            <button onclick="window.sceneManager.addSphere([Math.random()*2-1, 0, -1.5], 0.3, {type: 'lambertian', albedo: [Math.random(), Math.random(), Math.random()]})">Add Random Sphere</button><br/>
            <button onclick="console.log(window.sceneManager.getSceneJson())">Log Scene JSON</button><br/>
            <button onclick="window.location.reload()">Reload Scene</button>
        `
        document.body.appendChild(controls)

        // Start progressive rendering
        renderNextBatch()
    })
    .catch(console.error)
