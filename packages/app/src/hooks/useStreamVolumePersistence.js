import React from "react"

const STORE_KEY = "vc:stream_volumes"

export const useStreamVolumePersistence = () => {
	const [volumes, setVolumes] = React.useState(() => {
		try {
			if (typeof localStorage !== "undefined") {
				const item = localStorage.getItem(STORE_KEY)
				return item ? JSON.parse(item) : {}
			}
			return {}
		} catch {
			return {}
		}
	})

	const getVolume = React.useCallback(
		(userId) => {
			return volumes[userId] ?? 100
		},
		[volumes],
	)

	const setVolume = React.useCallback(
		(userId, volume) => {
			const newVolumes = { ...volumes, [userId]: volume }
			setVolumes(newVolumes)

			try {
				if (typeof localStorage !== "undefined") {
					localStorage.setItem(STORE_KEY, JSON.stringify(newVolumes))
				}
			} catch (error) {
				console.warn(
					"Failed to save stream volume to localStorage:",
					error,
				)
			}
		},
		[volumes],
	)

	const clearVolumes = React.useCallback(() => {
		setVolumes({})

		try {
			if (typeof localStorage !== "undefined") {
				localStorage.removeItem(STORE_KEY)
			}
		} catch (error) {
			console.warn(
				"Failed to clear stream volumes from localStorage:",
				error,
			)
		}
	}, [])

	return { getVolume, setVolume, clearVolumes, volumes }
}
