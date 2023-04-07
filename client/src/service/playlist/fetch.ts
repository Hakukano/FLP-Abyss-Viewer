import { listQueryToParams } from "@/service"
import * as Super from "../playlist"

export default class Playlist implements Super.Playlist {
  async read(path: Super.Read.Path): Promise<Super.Read.Response> {
    const res = await fetch(`/playlists/${path.id}`)
    if (res.ok) {
      return await res.json()
    } else {
      throw {
        status: res.status,
        message: await res.text(),
      } as Super.Error
    }
  }

  async list(query: Super.List.Query): Promise<Super.List.Response> {
    const res = await fetch(`/playlists?${listQueryToParams(query)}`)
    if (res.ok) {
      return await res.json()
    } else {
      throw {
        status: res.status,
        message: await res.text(),
      } as Super.Error
    }
  }
}
