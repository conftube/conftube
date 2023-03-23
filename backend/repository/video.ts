import {PrismaClient, User, Video} from "@prisma/client";

const prisma = new PrismaClient();

export const Videos = {
    findById: async (id: string): Promise<Video | null> => {
        return prisma.video.findFirst({where: {id: id}});
    },

    rate: async (video: Video, user: User, rating: number): Promise<Video | null> => {
        await prisma.rating.upsert({
            where: {
                videoId_userId: {
                    videoId: video?.id,
                    userId: user?.id
                }
            },
            update: {
                rating: rating
            },
            create: {
                videoId: video?.id,
                userId: user?.id,
                rating: rating
            }
        });

        const avgRating = await prisma.$queryRaw<[{avg: number}]>`SELECT AVG("rating") AS avg FROM "Rating" WHERE "videoId" = ${video.id}`;

        video.rating = avgRating[0].avg;

        await prisma.video.update({
            data: video,
            where: {
                id: video.id
            }
        });

        return video;
    }
}
