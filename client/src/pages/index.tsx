import { useState } from 'react'
import ReactPlayer from 'react-player'
import XMarkIcon from '@heroicons/react/24/outline/XMarkIcon'

import PlaylistRemote from '@/components/playlist_remote'
import Toolbar from '@/components/toolbar'
import ServicePlaylistRemote from '@/service/playlist_remote/fetch'
import { Data } from '@/service/playlist_remote'

export default function Page() {
  const playlistRemote = new ServicePlaylistRemote

  const [playlistData, setPlaylistData] = useState(null as Data | null)

  const [showPlaylistRemote, setShowPlaylistRemote] = useState(false)

  return (
    <main className='flex flex-col w-full h-full items-center justify-between p-2 space-y-2'>
      <Toolbar
        showPlaylistRemote={showPlaylistRemote}
        setShowPlaylistRemote={setShowPlaylistRemote}
      />
      <div className='w-full h-full'>
        {
          playlistData
            ? playlistData.mime_type.startsWith('image/')
              ? <></>
              : <ReactPlayer
                width='100%'
                height='88vh'
                url={`/playlists/${playlistData.id}/stream`}
                controls={true}
              />
            : <></>
        }
      </div>
      {
        showPlaylistRemote
          ?
          <>
            <div
              className="absolute justify-center items-center overflow-x-hidden overflow-y-auto fixed inset-0 z-50 outline-none focus:outline-none"
            >
              <div className="relative w-auto my-6 mx-auto max-w-3xl">
                {/*content*/}
                <div className="border-0 rounded-lg shadow-lg relative flex flex-col w-full bg-white outline-none focus:outline-none">
                  {/*header*/}
                  <div className="flex items-start justify-between p-1 border-b border-solid border-slate-200 rounded-t">
                    <h3 className="text-2xl text-black font-semibold">
                      Remote Playlist
                    </h3>
                    <XMarkIcon
                      className="h-8 p-1 ml-auto bg-transparent border-0 text-black float-right text-3xl leading-none font-semibold outline-none focus:outline-none"
                      onClick={() => setShowPlaylistRemote(false)}
                    />
                  </div>
                  {/*body*/}
                  <div className="relative p-2 flex-auto">
                    <PlaylistRemote
                      playlistRemote={playlistRemote}
                      setPlaylistData={setPlaylistData}
                    />
                  </div>
                </div>
              </div>
            </div>
            <div className="opacity-25 fixed inset-0 z-40 bg-black"></div>
          </>
          : <></>
      }
    </main>
  )
}
