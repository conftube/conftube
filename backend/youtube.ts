import {google} from "googleapis"


export async function searchOnYoutube(searchQuery: string, maxResults: number): Promise<YoutubeVideo[]> {
    const youtube = google.youtube({
        version: "v3",
        auth: process.env.YOUTUBE_API_KEY
    });

    const response = await youtube.search.list({
        part: ['snippet'],
        q: searchQuery + " conference",
        maxResults,
        safeSearch: 'strict'
    });

    return (await response.data.items ?? []).filter((e: any) => e.id!!.kind!! == 'youtube#video').map((e) => {
        return {
            id: e.id!!.videoId!!,
            platform: 'youtube',
            title: e.snippet!!.title!!,
            description: e.snippet!!.description!!,
            thumbnailUrl: e.snippet!!.thumbnails!!.medium!!.url!!,
            publishedAt: e.snippet!!.publishedAt!!
        }
    });
}

export async function findById(id: string): Promise<YoutubeVideo | null> {
    const youtube = google.youtube({
        version: "v3",
        auth: process.env.YOUTUBE_API_KEY
    });

    const response = await youtube.videos.list({
        part: ['id', 'snippet'],
        id: [id]
    });

    if ((response.data.items ?? []).length == 0) {
        return null;
    }

    const e = response.data.items!![0];

    return {
        id: e.id!!,
        platform: 'youtube',
        title: e.snippet!!.title!!,
        description: e.snippet!!.description!!,
        thumbnailUrl: e.snippet!!.thumbnails!!.medium!!.url!!,
        publishedAt: e.snippet!!.publishedAt!!
    }
}

export type YoutubeVideo = {
    id: string
    platform: 'youtube'
    title: string,
    description: string,
    thumbnailUrl: string,
    publishedAt: string
}
