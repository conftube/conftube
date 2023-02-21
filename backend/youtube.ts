import {google} from "googleapis"


export async function searchOnYoutube(searchQuery: string, maxResults: number) {
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

    return (await response.data.items ?? []).filter((e: any) => e.id!!.kind!! == 'youtube#video')
}
