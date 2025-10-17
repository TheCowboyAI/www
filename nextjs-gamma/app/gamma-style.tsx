'use client'
import {
  Box,
  Container,
  Heading,
  Text,
  VStack,
  HStack,
  Card,
  CardBody,
  Grid,
  Flex,
  Circle,
  Button,
  Image,
  useColorMode,
  Table,
  Thead,
  Tbody,
  Tr,
  Th,
  Td,
} from '@chakra-ui/react'

export default function GammaPresentation() {
  return (
    <Box minH="100vh">
      {/* Navigation */}
      <Flex
        position="fixed"
        top={0}
        left={0}
        right={0}
        bg="white"
        p={4}
        zIndex={100}
        align="center"
        justify="space-between"
        boxShadow="sm"
      >
        <Flex align="center" gap={2}>
          <Image src="/logo.svg" h={10} alt="Logo" />
        </Flex>
        <HStack spacing={4}>
          <Button variant="ghost" color="gray.600">HOME</Button>
          <Button bg="blue.500" color="white" _hover={{ bg: "blue.600" }}>UI DEMO</Button>
          <Button bg="blue.500" color="white" _hover={{ bg: "blue.600" }}>TEAM</Button>
        </HStack>
      </Flex>

      {/* Slide 1: Hero */}
      <Flex
        minH="100vh"
        bgGradient="linear(135deg, #667eea 0%, #764ba2 100%)"
        bgImage="url('https://images.unsplash.com/photo-1635070041078-e363dbe005cb?w=1920')"
        bgBlendMode="soft-light"
        bgSize="cover"
        bgPos="center"
        align="center"
        justify="center"
        position="relative"
      >
        <Box
          position="absolute"
          inset={0}
          bg="rgba(102, 126, 234, 0.7)"
          backdropFilter="blur(2px)"
        />
        <Container maxW="1200px" position="relative">
          <Card
            bg="white"
            borderRadius="3xl"
            p={16}
            boxShadow="2xl"
            maxW="900px"
            mx="auto"
          >
            <CardBody>
              <VStack spacing={6} textAlign="center">
                <Heading size="4xl" color="blue.600">
                  Cowboy AI
                </Heading>
                <Heading size="xl" color="gray.700">
                  The Platform For Composable, Cognitive, Audit-Grade AI Swarms
                </Heading>
                <Text fontSize="lg" color="gray.600">
                  Your New Business Brain & Nervous System. Cognition. Evolution. Security. Trust.
                </Text>
                <Text fontSize="lg" color="gray.600">
                  No-Code Composable Multi-AI Agent Orchestration At Scale
                </Text>
                <Box
                  mt={8}
                  p={6}
                  bg="gray.50"
                  borderRadius="xl"
                  borderLeft="4px solid"
                  borderColor="blue.500"
                >
                  <Text fontWeight="bold" mb={2}>Disclaimer & Warning:</Text>
                  <Text>You can now ask and command the system to do anything. In plain English. And it will.</Text>
                </Box>
              </VStack>
            </CardBody>
          </Card>
        </Container>
      </Flex>

      {/* Slide 2: No Cognition */}
      <Flex
        minH="100vh"
        bgGradient="linear(135deg, #667eea 0%, #764ba2 100%)"
        bgImage="url('https://images.unsplash.com/photo-1620712943543-bcc4688e7485?w=1920')"
        bgBlendMode="soft-light"
        bgSize="cover"
        bgPos="center"
        align="center"
        justify="center"
        position="relative"
      >
        <Box
          position="absolute"
          inset={0}
          bg="rgba(102, 126, 234, 0.7)"
          backdropFilter="blur(2px)"
        />
        <Container maxW="1200px" position="relative">
          <Card bg="white" borderRadius="3xl" p={12} boxShadow="2xl">
            <CardBody>
              <VStack spacing={8}>
                <Heading size="2xl" color="blue.600" textAlign="center">
                  No Cognition? - No AGI!
                </Heading>
                <Text fontSize="lg" textAlign="center" color="gray.600">
                  Intelligence Alone Isn't Enough—Cognition Is the Missing Piece.
                </Text>
                
                {/* Circular Segmented Diagram */}
                <Box position="relative" w="400px" h="400px" mx="auto">
                  <svg viewBox="0 0 400 400" width="100%" height="100%">
                    {/* Segment 1 - Hardware */}
                    <path
                      d="M 200 200 L 200 50 A 150 150 0 0 1 350 200 Z"
                      fill="#10b981"
                      stroke="white"
                      strokeWidth="3"
                    />
                    <text x="275" y="125" fill="white" fontSize="24" fontWeight="bold">1</text>
                    
                    {/* Segment 2 - LLM */}
                    <path
                      d="M 200 200 L 350 200 A 150 150 0 0 1 200 350 Z"
                      fill="#10b981"
                      stroke="white"
                      strokeWidth="3"
                    />
                    <text x="275" y="275" fill="white" fontSize="24" fontWeight="bold">2</text>
                    
                    {/* Segment 3 - Cognition */}
                    <path
                      d="M 200 200 L 200 350 A 150 150 0 0 1 50 200 Z"
                      fill="#3b82f6"
                      stroke="white"
                      strokeWidth="3"
                    />
                    <text x="125" y="275" fill="white" fontSize="24" fontWeight="bold">3</text>
                    
                    {/* Segment 4 - Your World */}
                    <path
                      d="M 200 200 L 50 200 A 150 150 0 0 1 200 50 Z"
                      fill="#3b82f6"
                      stroke="white"
                      strokeWidth="3"
                    />
                    <text x="125" y="125" fill="white" fontSize="24" fontWeight="bold">4</text>
                    
                    {/* Center circle with brain icon */}
                    <circle cx="200" cy="200" r="60" fill="white" stroke="#e5e7eb" strokeWidth="2"/>
                    <text x="200" y="210" textAnchor="middle" fontSize="40">🧠</text>
                  </svg>
                  
                  {/* Labels */}
                  <Box position="absolute" top="20px" left="50%" transform="translateX(-50%)">
                    <Text fontWeight="bold">Hardware</Text>
                    <Text fontSize="sm" color="gray.600">Horsepower</Text>
                  </Box>
                  <Box position="absolute" top="50%" right="20px" transform="translateY(-50%)">
                    <Text fontWeight="bold">LLM</Text>
                    <Text fontSize="sm" color="gray.600">Raw tooling</Text>
                  </Box>
                  <Box position="absolute" bottom="20px" left="50%" transform="translateX(-50%)">
                    <Text fontWeight="bold">Cognition</Text>
                    <Text fontSize="sm" color="gray.600">Awareness of tools & State</Text>
                  </Box>
                  <Box position="absolute" top="50%" left="20px" transform="translateY(-50%)">
                    <Text fontWeight="bold">Your World - The Data</Text>
                    <Text fontSize="sm" color="gray.600" maxW="150px">Clean, Holistic, Accessible, Mathematically Proven</Text>
                  </Box>
                </Box>

                <VStack spacing={4} mt={8}>
                  <Text fontSize="lg" textAlign="center">
                    For true AGI, systems must <strong>learn, adapt, and evolve</strong>—not just process data.
                  </Text>
                  <Text textAlign="center" color="gray.600">
                    Today's AI is trapped in silos, built on imperfect math, with no structure or guardrails.
                  </Text>
                  <Text fontSize="xl" fontWeight="bold" textAlign="center">
                    That's why it keeps failing.
                  </Text>
                  <Text fontSize="xl" fontWeight="bold" color="blue.600" textAlign="center">
                    We solved it.
                  </Text>
                  <Text textAlign="center" color="gray.600">
                    By embedding <strong>cognition at the core</strong>, Cowboy AI creates an adaptive, composable 
                    architecture that continuously learns, evolves, and safeguards decision-making.
                  </Text>
                  <Text fontSize="xl" fontWeight="bold" textAlign="center">
                    This is the breakthrough that makes AGI possible.
                  </Text>
                </VStack>
              </VStack>
            </CardBody>
          </Card>
        </Container>
      </Flex>

      {/* Slide 3: Platform First */}
      <Flex
        minH="100vh"
        bgGradient="linear(135deg, #f093fb 0%, #f5576c 100%)"
        align="center"
        justify="center"
        position="relative"
      >
        <Container maxW="1200px">
          <Card bg="white" borderRadius="3xl" p={12} boxShadow="2xl">
            <CardBody>
              <VStack spacing={8}>
                <Heading size="2xl" color="purple.600" textAlign="center">
                  Platform First — Cowboy AI CIM
                </Heading>
                <Text fontSize="lg" textAlign="center" color="gray.600">
                  The platform for building, governing, and scaling your business with AI swarms.
                </Text>
                
                <Grid templateColumns={{ base: "1fr", md: "repeat(2, 1fr)", lg: "repeat(3, 1fr)" }} gap={6}>
                  {[
                    {
                      title: "🌐 Build Once, Replicate Anywhere",
                      text: "We're proving it in private lending/fintech first. Repeatable & Reproduceable.",
                      img: "https://images.unsplash.com/photo-1551288049-bebda4e38f71?w=400"
                    },
                    {
                      title: "🔧 Open for Builders",
                      text: "Publish domain packs, swarms, and connectors other teams can adopt.",
                      img: "https://images.unsplash.com/photo-1605810230434-7631ac76ec81?w=400"
                    },
                    {
                      title: "🏆 Proven Milestones",
                      text: "8M+ micro-transactions processed; hybrid runtime targeting >90% lower AI compute vs cloud-only baselines.",
                      img: "https://images.unsplash.com/photo-1460925895917-afdab827c52f?w=400"
                    },
                    {
                      title: "🤖 No-code, modular agent swarms",
                      text: "Like little building blocks AI agents assemble and compose on the fly to execute any task.",
                      img: "https://images.unsplash.com/photo-1485827404703-89b55fcc595e?w=400"
                    },
                    {
                      title: "🔒 Immutable, Security First",
                      text: "Immutable records, ransomware-resistant architecture",
                      img: "https://images.unsplash.com/photo-1558494949-ef010cbdcc31?w=400"
                    }
                  ].map((item, idx) => (
                    <Card key={idx} borderRadius="2xl" overflow="hidden" boxShadow="lg">
                      <Box h="150px" bgImage={`url('${item.img}')`} bgSize="cover" bgPos="center" position="relative">
                        <Box position="absolute" inset={0} bg="rgba(0,0,0,0.4)" />
                      </Box>
                      <CardBody p={6}>
                        <Heading size="md" mb={3}>{item.title}</Heading>
                        <Text color="gray.600">{item.text}</Text>
                      </CardBody>
                    </Card>
                  ))}
                </Grid>
              </VStack>
            </CardBody>
          </Card>
        </Container>
      </Flex>

      {/* Slide 4: Team */}
      <Flex
        minH="100vh"
        bgGradient="linear(135deg, #667eea 0%, #764ba2 100%)"
        align="center"
        justify="center"
        position="relative"
        py={20}
      >
        <Container maxW="1400px">
          <Card bg="white" borderRadius="3xl" p={12} boxShadow="2xl">
            <CardBody>
              <VStack spacing={8}>
                <Heading size="2xl" color="blue.600" textAlign="center">
                  Our Leadership Team
                </Heading>
                
                <Grid templateColumns={{ base: "1fr", md: "repeat(2, 1fr)", lg: "repeat(3, 1fr)" }} gap={6}>
                  {[
                    {
                      name: "Jacob Kopilovitch - CEO",
                      bio: "Serial entrepreneur with over 20 years of experience driving strategic vision, business growth, and organizational leadership across multiple industries.",
                      img: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=400"
                    },
                    {
                      name: "David Kopilovitch - COO",
                      bio: "Operations executive with 17+ years of expertise in back office management, process optimization, and detailed execution of complex business operations.",
                      img: "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400"
                    },
                    {
                      name: "Steele Price - CSO",
                      bio: "Technology visionary with 40+ years of experience, including 10 years as Microsoft MVP and core team member for .NET language development.",
                      img: "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400"
                    },
                    {
                      name: "Kathryn Freeman - CBO",
                      bio: "Accomplished entrepreneur and senior advisor with 17+ years guiding fintech and lending companies.",
                      img: "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?w=400"
                    },
                    {
                      name: "Mark Sonnenklar - CLO",
                      bio: "Premier business and intellectual property attorney with 25+ years of experience in corporate and private law.",
                      img: "https://images.unsplash.com/photo-1519345182560-3f2917c472ef?w=400"
                    },
                    {
                      name: "Ryan Plemons - Senior Architect",
                      bio: "Veteran technology leader with 30+ years as a senior engineer, specializing in AI implementation.",
                      img: "https://images.unsplash.com/photo-1463453091185-61582044d556?w=400"
                    }
                  ].map((member, idx) => (
                    <Card key={idx} borderRadius="2xl" overflow="hidden" boxShadow="lg">
                      <Box h="200px" bgImage={`url('${member.img}')`} bgSize="cover" bgPos="center" />
                      <CardBody p={6}>
                        <Heading size="md" mb={3} color="blue.700">{member.name}</Heading>
                        <Text color="gray.600" fontSize="sm">{member.bio}</Text>
                      </CardBody>
                    </Card>
                  ))}
                </Grid>
              </VStack>
            </CardBody>
          </Card>
        </Container>
      </Flex>
    </Box>
  )
}