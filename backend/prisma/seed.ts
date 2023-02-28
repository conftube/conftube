import { PrismaClient } from '@prisma/client'
const prisma = new PrismaClient()

async function main() {
    const admin = await prisma.user.upsert({
        where: { email: 'test@example.com' },
        update: {},
        create: {
            email: 'test@example.com',
            givenName: 'Test',
            familyName: 'Example',
            picture: 'https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRdXF9CU0PeppjYjQkOuY-o4z3JBwMWM3fPcg&usqp=CAU'
        },
    })

    console.log({ admin })
}
main()
    .then(async () => {
        await prisma.$disconnect()
    })
    .catch(async (e) => {
        console.error(e)
        await prisma.$disconnect()
        process.exit(1)
    })
