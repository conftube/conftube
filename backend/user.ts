import {PrismaClient} from "@prisma/client";
import {Context} from "./index";

const prisma = new PrismaClient();

export async function profile(_: any, context: Context) {
    const user = context.user;

    await prisma.user.upsert({
        create: {
            email: user.email,
            givenName: user.given_name,
            familyName: user.family_name,
            picture: user.picture
        },
        update: {
            givenName: user.given_name,
            familyName: user.family_name,
            picture: user.picture
        },
        where: {
            email: user.email
        }
    });

    return prisma.user.findFirst({
        where: {
            email: context.user.email
        }
    });
}
