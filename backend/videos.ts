import {searchOnYoutube} from "./youtube";
import {PrismaClient} from "@prisma/client";

const prisma = new PrismaClient();

type SearchVideoResult = {
    id: string,
    title: string,
    description: string,
    publishedAt: string,
    platform: string
}

export type SearchVideoParams = {
    title: string,
    maxResults?: number
}

export async function searchVideos(params: SearchVideoParams): Promise<Array<SearchVideoResult>> {
    const maxResults = params.maxResults ?? 20;
    const youtubeSearch = searchOnYoutube(params.title, maxResults);

    const internal: Array<SearchVideoResult> = (await prisma.video.findMany({
        take: maxResults,
        where: {
            title: {
                search: params.title
            }
        },
        orderBy: {
            publishedAt: 'desc'
        },
        include: {
            owner: true
        }
    })).map(e => {
        return {
            id: e.id,
            platform: e.platform,
            title: e.title,
            description: e.description,
            publishedAt: e.publishedAt.toString()
        }
    });

    const external: Array<SearchVideoResult> = ((await youtubeSearch) ?? []).map(e => {
        return {
            id: e.id!!.videoId!!,
            platform: 'youtube',
            title: e.snippet!!.title!!,
            description: e.snippet!!.description!!,
            publishedAt: e.snippet!!.publishedAt!!
        }
    });

    return internal.concat(external).slice(0, maxResults)
}
