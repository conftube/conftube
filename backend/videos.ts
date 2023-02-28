import {findById, searchOnYoutube, YoutubeVideo} from "./youtube";
import {PrismaClient, Video} from "@prisma/client";
import {GraphQLError} from "graphql";

const prisma = new PrismaClient();

export type PaginatedVideosParams = {
    id: string,
    first: number,
    offset: number
}

export async function videos(params: PaginatedVideosParams) {
    const offset = params.offset ?? 0;
    const first = params.first ?? 25;

    const filters = {
        id: params.id,
    }

    const total = await prisma.video.count();
    const results = await prisma.video.findMany({
        skip: offset,
        take: first,
        where: filters
    });

    return {
        first,
        offset,
        total,
        results
    }
}

export type SearchVideoParams = {
    query: string,
    maxResults?: number
}

export async function searchVideos(params: SearchVideoParams): Promise<Array<Video|YoutubeVideo>> {
    const maxResults = params.maxResults ?? 20;
    const youtubeSearch = searchOnYoutube(params.query, maxResults);

    const internal: Array<Video> = await prisma.$queryRaw`SELECT * FROM "Video" WHERE "Video".title LIKE '%${params.query}%' LIMIT ${maxResults}`;
    const external: Array<YoutubeVideo> = (await youtubeSearch) ?? [];

    return [...internal, ...external].slice(0, maxResults)
}

export type AddVideoInput = {
    id: string,
    platform: string
}

export async function addVideo(input: AddVideoInput): Promise<Video|GraphQLError> {
    const video = await findById(input.id);

    if (video === null) {
        return new GraphQLError(`Could not find video with ID ${input.id}`)
    }

    return prisma.video.upsert({
        where: {
            id: video?.id,
        },
        update: {
            description: video?.description,
            title: video?.title,
            thumbnailUrl: video?.thumbnailUrl,
        },
        create: {
            id: video?.id,
            description: video?.description,
            platform: 'youtube',
            title: video?.title,
            thumbnailUrl: video?.thumbnailUrl,
            publishedAt: video?.publishedAt
        }
    });
}
