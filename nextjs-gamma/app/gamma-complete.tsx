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
  Table,
  Thead,
  Tbody,
  Tr,
  Th,
  Td,
  TableContainer,
  List,
  ListItem,
  Stack,
  Badge,
  Divider,
  useColorModeValue,
} from '@chakra-ui/react'

const slides = [
  {
    id: 'hero',
    background: 'linear(135deg, #667eea 0%, #764ba2 100%)',
    bgImage: 'https://images.unsplash.com/photo-1635070041078-e363dbe005cb?w=1920',
  },
  {
    id: 'cognition',
    background: 'linear(135deg, #667eea 0%, #764ba2 100%)',
    bgImage: 'https://images.unsplash.com/photo-1620712943543-bcc4688e7485?w=1920',
  },
  {
    id: 'ai-breaking',
    background: 'linear(135deg, #f093fb 0%, #f5576c 100%)',
  },
  {
    id: 'platform',
    background: 'linear(135deg, #4facfe 0%, #00f2fe 100%)',
  },
  {
    id: 'stack',
    background: 'linear(135deg, #667eea 0%, #764ba2 100%)',
  },
  {
    id: 'breakthrough',
    background: 'linear(135deg, #fa709a 0%, #fee140 100%)',
  },
  {
    id: 'trust',
    background: 'linear(135deg, #30cfd0 0%, #330867 100%)',
  },
  {
    id: 'sage',
    background: 'linear(135deg, #a8edea 0%, #fed6e3 100%)',
  },
]

function CircularCard({ number, title, subtitle, color = '#10b981', image }: any) {
  return (
    <Flex direction="column" align="center" gap={3}>
      <Box position="relative" w="180px" h="180px">
        <Box
          w="full"
          h="full"
          borderRadius="full"
          bg={color}
          opacity={0.2}
        />
        <Box
          position="absolute"
          inset="4"
          borderRadius="full"
          bgImage={image ? `url('${image}')` : `linear-gradient(135deg, ${color} 0%, ${color}88 100%)`}
          bgSize="cover"
          bgPos="center"
        />
        <Circle
          size="50px"
          bg={color}
          color="white"
          position="absolute"
          bottom="10px"
          right="10px"
          fontSize="24px"
          fontWeight="bold"
          border="3px solid white"
          boxShadow="0 4px 6px rgba(0,0,0,0.2)"
        >
          {number}
        </Circle>
      </Box>
      <VStack spacing={1} textAlign="center">
        <Text fontWeight="bold" fontSize="lg">{title}</Text>
        <Text fontSize="sm" color="gray.600" maxW="200px">{subtitle}</Text>
      </VStack>
    </Flex>
  )
}

export default function GammaComplete() {
  return (
    <Box minH="100vh" bg="black">
      {/* Navigation - Fixed */}
      <Flex
        position="fixed"
        top={0}
        left={0}
        right={0}
        bg="white"
        p={4}
        zIndex={1000}
        align="center"
        justify="space-between"
        boxShadow="md"
      >
        <Text fontSize="2xl" fontWeight="bold" color="blue.600">🤠 Cowboy AI</Text>
        <HStack spacing={2}>
          <Button size="sm" variant="ghost">HOME</Button>
          <Button size="sm" colorScheme="blue">UI DEMO</Button>
          <Button size="sm" colorScheme="blue">TEAM</Button>
        </HStack>
      </Flex>

      {/* Slide 1: Hero */}
      <Flex
        minH="100vh"
        bgGradient={slides[0].background}
        bgImage={`url('${slides[0].bgImage}')`}
        bgBlendMode="soft-light"
        bgSize="cover"
        bgPos="center"
        align="center"
        justify="center"
        position="relative"
        pt={16}
      >
        <Box position="absolute" inset={0} bg="rgba(102, 126, 234, 0.8)" />
        <Container maxW="1200px" position="relative">
          <Card bg="white" borderRadius="3xl" p={{ base: 8, md: 16 }} boxShadow="2xl">
            <CardBody>
              <VStack spacing={8} textAlign="center">
                <Heading size="4xl" color="blue.600" fontWeight="900">
                  Cowboy AI
                </Heading>
                <Heading size="xl" color="gray.700" fontWeight="600">
                  The Platform For Composable, Cognitive, Audit‑Grade AI Swarms
                </Heading>
                <Text fontSize="lg" color="gray.600">
                  Your New Business Brain & Nervous System. Cognition. Evolution. Security. Trust.
                </Text>
                <Text fontSize="lg" fontWeight="bold" color="gray.700">
                  No‑Code Composable Multi‑AI Agent Orchestration At Scale
                </Text>
                <Box mt={8} p={6} bg="blue.50" borderRadius="xl" borderLeft="4px solid" borderColor="blue.500">
                  <Text fontWeight="bold" mb={2}>Disclaimer & Warning:</Text>
                  <Text>You can now ask and command the system to do anything. In plain English. And it will.</Text>
                </Box>
              </VStack>
            </CardBody>
          </Card>
        </Container>
      </Flex>

      {/* Slide 2: No Cognition - No AGI */}
      <Flex
        minH="100vh"
        bgGradient={slides[1].background}
        bgImage={`url('${slides[1].bgImage}')`}
        bgBlendMode="soft-light"
        bgSize="cover"
        align="center"
        justify="center"
        position="relative"
      >
        <Box position="absolute" inset={0} bg="rgba(102, 126, 234, 0.8)" />
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
                
                <Grid templateColumns="repeat(2, 1fr)" gap={8} mt={8} justifyItems="center">
                  <CircularCard 
                    number="1" 
                    title="Hardware" 
                    subtitle="Horsepower" 
                    color="#10b981"
                    image="https://images.unsplash.com/photo-1518770660439-4636190af475?w=400" 
                  />
                  <CircularCard 
                    number="2" 
                    title="LLM" 
                    subtitle="Raw tooling" 
                    color="#10b981"
                    image="https://images.unsplash.com/photo-1677442136019-21780ecad995?w=400" 
                  />
                  <CircularCard 
                    number="3" 
                    title="Cognition" 
                    subtitle="Awareness of tools & State" 
                    color="#3b82f6"
                    image="https://images.unsplash.com/photo-1620712943543-bcc4688e7485?w=400" 
                  />
                  <CircularCard 
                    number="4" 
                    title="Your World - The Data" 
                    subtitle="Clean, Holistic, Accessible, Mathematically Proven" 
                    color="#3b82f6"
                    image="https://images.unsplash.com/photo-1451187580459-43490279c0fa?w=400" 
                  />
                </Grid>

                <VStack spacing={4} mt={8}>
                  <Text fontSize="lg" textAlign="center">
                    For true AGI, systems must <strong>learn, adapt, and evolve</strong>—not just process data.
                  </Text>
                  <Text textAlign="center" color="gray.600">
                    Today's AI is trapped in silos, built on imperfect math, with no structure or guardrails.
                  </Text>
                  <Text fontSize="xl" fontWeight="bold" textAlign="center">That's why it keeps failing.</Text>
                  <Text fontSize="2xl" fontWeight="bold" color="blue.600" textAlign="center">We solved it.</Text>
                  <Text textAlign="center" maxW="800px">
                    By embedding <strong>cognition at the core</strong>, Cowboy AI creates an adaptive, 
                    composable architecture that continuously learns, evolves, and safeguards decision-making.
                  </Text>
                  <Box p={6} bg="blue.50" borderRadius="xl" mt={4}>
                    <Text fontSize="xl" fontWeight="bold" textAlign="center">
                      This is the breakthrough that makes AGI possible.
                    </Text>
                  </Box>
                </VStack>
              </VStack>
            </CardBody>
          </Card>
        </Container>
      </Flex>

      {/* Slide 3: AI Is Breaking at Scale */}
      <Flex
        minH="100vh"
        bgGradient={slides[2].background}
        align="center"
        justify="center"
        position="relative"
      >
        <Container maxW="1200px">
          <Card bg="white" borderRadius="3xl" p={12} boxShadow="2xl">
            <CardBody>
              <VStack spacing={8}>
                <Heading size="2xl" color="purple.600" textAlign="center">
                  AI Is Breaking at Scale
                </Heading>
                
                <Grid templateColumns={{ base: "1fr", md: "repeat(3, 1fr)" }} gap={6}>
                  <Card borderRadius="xl" p={6} boxShadow="lg" borderTop="4px solid" borderColor="purple.500">
                    <VStack align="start" spacing={3}>
                      <Heading size="md">🚧 Fragmentation kills velocity</Heading>
                      <Text color="gray.600">
                        Businesses juggle many applications & tools, re‑enter data, and never achieve true sync.
                      </Text>
                    </VStack>
                  </Card>
                  <Card borderRadius="xl" p={6} boxShadow="lg" borderTop="4px solid" borderColor="red.500">
                    <VStack align="start" spacing={3}>
                      <Heading size="md">💣 The last 10% explodes compute</Heading>
                      <Text color="gray.600">
                        Teams reach 85–90% automation, then costs and fragility spike without the right structures.
                      </Text>
                    </VStack>
                  </Card>
                  <Card borderRadius="xl" p={6} boxShadow="lg" borderTop="4px solid" borderColor="orange.500">
                    <VStack align="start" spacing={3}>
                      <Heading size="md">🔒 Lock‑in & opacity</Heading>
                      <Text color="gray.600">
                        Cloud‑centric AI → runaway spend, brittle workflows, and audits no one can pass.
                      </Text>
                    </VStack>
                  </Card>
                </Grid>

                <TableContainer w="full" mt={8}>
                  <Table variant="simple">
                    <Thead bg="gray.50">
                      <Tr>
                        <Th>Root Cause</Th>
                        <Th>Symptom</Th>
                      </Tr>
                    </Thead>
                    <Tbody>
                      <Tr>
                        <Td fontWeight="bold">Fragmented & proprietary stacks</Td>
                        <Td>Siloed data → duplication waste and a hard throughput ceiling.</Td>
                      </Tr>
                      <Tr bg="gray.50">
                        <Td fontWeight="bold">Vendor lock-in & cloud-only ops</Td>
                        <Td>Exploding costs at scale; switching costs paralyze change.</Td>
                      </Tr>
                      <Tr>
                        <Td fontWeight="bold">Brittle translations/APIs</Td>
                        <Td>Scale-load failures; throttling/egress fees bleed budgets.</Td>
                      </Tr>
                      <Tr bg="gray.50">
                        <Td fontWeight="bold">Imperfect-math / nondeterministic code paths</Td>
                        <Td>Inconsistent outputs; expensive debugging caps reliability.</Td>
                      </Tr>
                      <Tr>
                        <Td fontWeight="bold">Weak governance & no single source of truth</Td>
                        <Td>Untraceable decisions; audit exposure inflates compliance spend.</Td>
                      </Tr>
                      <Tr bg="gray.50">
                        <Td fontWeight="bold">Scale threshold ("last 10%")</Td>
                        <Td>Compute spikes and instability; ROI collapses at ~85–90% automation.</Td>
                      </Tr>
                    </Tbody>
                  </Table>
                </TableContainer>
              </VStack>
            </CardBody>
          </Card>
        </Container>
      </Flex>

      {/* Slide 4: Platform First */}
      <Flex
        minH="100vh"
        bgGradient={slides[3].background}
        align="center"
        justify="center"
        position="relative"
      >
        <Container maxW="1400px">
          <Card bg="white" borderRadius="3xl" p={12} boxShadow="2xl">
            <CardBody>
              <VStack spacing={8}>
                <Heading size="2xl" color="blue.600" textAlign="center">
                  Platform First — Cowboy AI CIM
                </Heading>
                <Text fontSize="lg" textAlign="center" color="gray.600">
                  The platform for building, governing, and scaling your business with AI swarms.
                </Text>
                
                <Grid templateColumns={{ base: "1fr", md: "repeat(3, 1fr)", lg: "repeat(5, 1fr)" }} gap={6}>
                  {[
                    { num: "1", title: "Build Once, Replicate Anywhere", text: "We're proving it in private lending/fintech first. Repeatable & Reproduceable." },
                    { num: "2", title: "Open for Builders", text: "Publish domain packs, swarms, and connectors other teams can adopt." },
                    { num: "3", title: "Proven Milestones", text: "8M+ micro-transactions processed; >90% lower AI compute vs cloud-only." },
                    { num: "4", title: "No‑code, modular agent swarms", text: "Like little building blocks AI agents assemble on the fly." },
                    { num: "5", title: "Immutable, Security First", text: "Immutable records, ransomware-resistant architecture" },
                  ].map((item, idx) => (
                    <Box key={idx} position="relative">
                      <Box
                        h="200px"
                        borderRadius="2xl"
                        bgImage={`url('https://images.unsplash.com/photo-${1550000000000 + idx * 1000000000}?w=400')`}
                        bgSize="cover"
                        bgPos="center"
                        position="relative"
                        overflow="hidden"
                      >
                        <Box position="absolute" inset={0} bg="rgba(0,0,0,0.4)" />
                        <Circle
                          size="60px"
                          bg="white"
                          color="blue.600"
                          position="absolute"
                          top="50%"
                          left="50%"
                          transform="translate(-50%, -50%)"
                          fontSize="28px"
                          fontWeight="bold"
                          boxShadow="xl"
                        >
                          {item.num}
                        </Circle>
                      </Box>
                      <VStack mt={4} spacing={2}>
                        <Text fontWeight="bold" fontSize="lg" textAlign="center">{item.title}</Text>
                        <Text fontSize="sm" color="gray.600" textAlign="center">{item.text}</Text>
                      </VStack>
                    </Box>
                  ))}
                </Grid>
              </VStack>
            </CardBody>
          </Card>
        </Container>
      </Flex>

      {/* Slide 5: Who's Who in the Stack */}
      <Flex
        minH="100vh"
        bgGradient={slides[4].background}
        align="center"
        justify="center"
        position="relative"
      >
        <Box position="absolute" inset={0} bg="rgba(102, 126, 234, 0.8)" />
        <Container maxW="1200px" position="relative">
          <Card bg="white" borderRadius="3xl" p={12} boxShadow="2xl">
            <CardBody>
              <VStack spacing={8}>
                <Heading size="2xl" color="blue.600" textAlign="center">
                  Who's Who in the Stack
                </Heading>
                
                <Grid templateColumns={{ base: "1fr", md: "repeat(3, 1fr)" }} gap={8}>
                  <Box>
                    <CircularCard 
                      number="1" 
                      title="SAGE - Agent Orchestrator" 
                      subtitle="The Brain - Composes and drives swarms of AIs" 
                    />
                  </Box>
                  <Box>
                    <CircularCard 
                      number="2" 
                      title="Alchemist Ontology" 
                      subtitle="The Nervous System - Your real-time knowledge graph" 
                    />
                  </Box>
                  <Box>
                    <CircularCard 
                      number="3" 
                      title="CIM - Platform" 
                      subtitle="The Body - Your Composable Information Machine" 
                    />
                  </Box>
                </Grid>
              </VStack>
            </CardBody>
          </Card>
        </Container>
      </Flex>

      {/* Slide 6: Our Breakthrough */}
      <Flex
        minH="100vh"
        bgGradient={slides[5].background}
        align="center"
        justify="center"
        position="relative"
        py={20}
      >
        <Container maxW="1400px">
          <Card bg="white" borderRadius="3xl" p={12} boxShadow="2xl">
            <CardBody>
              <VStack spacing={8}>
                <Heading size="2xl" color="orange.600" textAlign="center">
                  Our Breakthrough — Swarm AI that actually scales
                </Heading>
                
                <Grid templateColumns={{ base: "1fr", md: "repeat(3, 1fr)" }} gap={6}>
                  {[
                    { icon: "🔄", title: "Repeatable", desc: "Commutative-diagram checks ensure same outcomes" },
                    { icon: "📊", title: "Explainable & Auditable", desc: "Every state change is an immutable event" },
                    { icon: "🔄", title: "Model-Hot-Swap", desc: "Switch between Claude/GPT/Llama without losing state" },
                    { icon: "💰", title: "90% Cheaper", desc: "Hybrid runtime + intelligent routing" },
                    { icon: "🧩", title: "Modular Building Blocks", desc: "Atomic functions → Extreme reuse" },
                    { icon: "🚀", title: "AI Amplification", desc: "Every employee gets an AI team on the fly" },
                  ].map((item, idx) => (
                    <Card key={idx} borderRadius="xl" p={6} boxShadow="lg" borderLeft="4px solid" borderColor="orange.400">
                      <VStack align="start" spacing={3}>
                        <Text fontSize="3xl">{item.icon}</Text>
                        <Heading size="md">{item.title}</Heading>
                        <Text color="gray.600">{item.desc}</Text>
                      </VStack>
                    </Card>
                  ))}
                </Grid>
              </VStack>
            </CardBody>
          </Card>
        </Container>
      </Flex>

      {/* Slide 7: Trust by Design */}
      <Flex
        minH="100vh"
        bgGradient={slides[6].background}
        align="center"
        justify="center"
        position="relative"
      >
        <Box position="absolute" inset={0} bg="rgba(48, 207, 208, 0.8)" />
        <Container maxW="1400px" position="relative">
          <Card bg="white" borderRadius="3xl" p={12} boxShadow="2xl">
            <CardBody>
              <VStack spacing={8}>
                <VStack spacing={2}>
                  <Heading size="2xl" color="teal.700" textAlign="center">
                    Trust by Design:
                  </Heading>
                  <Heading size="lg" color="gray.600" textAlign="center">
                    Security, Control, and Explainability at the Core
                  </Heading>
                  <Text textAlign="center" color="gray.600" maxW="800px">
                    Cowboy AI delivers secure-by-default automation through a modern, composable architecture 
                    engineered for financial-grade integrity and adaptability—on your terms.
                  </Text>
                </VStack>
                
                <Grid templateColumns={{ base: "1fr", md: "repeat(2, 1fr)" }} gap={6}>
                  {[
                    {
                      icon: "🏗️",
                      title: "Infrastructure of Integrity",
                      items: [
                        "Apple M3 Ultra + NixDarwin for deterministic environments",
                        "Rust + Bevy Runtime for memory-safe orchestration",
                        "NATS Messaging for real-time, fault-tolerant communication"
                      ]
                    },
                    {
                      icon: "🔐",
                      title: "Security That Scales",
                      items: [
                        "YubiKey Hardware Auth + ECC Encryption",
                        "MinIO + Wasabi immutable storage",
                        "AI Model Agnostic deployment"
                      ]
                    },
                    {
                      icon: "🧠",
                      title: "Explainable Intelligence",
                      items: [
                        "Neo4j Graph Engine + Event Sourcing",
                        "Functional Programming + TDD",
                        "DDD + Conceptual Spaces"
                      ]
                    },
                    {
                      icon: "🔢",
                      title: "Mathematics of Trust",
                      items: [
                        "Category Theory & Graph Theory",
                        "Gradient Descent Programming",
                        "Trust proven line by line"
                      ]
                    }
                  ].map((section, idx) => (
                    <Card key={idx} borderRadius="xl" p={6} bg="gray.50">
                      <VStack align="start" spacing={4}>
                        <HStack>
                          <Text fontSize="2xl">{section.icon}</Text>
                          <Heading size="md">{section.title}</Heading>
                        </HStack>
                        <List spacing={2}>
                          {section.items.map((item, i) => (
                            <ListItem key={i}>
                              <Text fontSize="sm">• {item}</Text>
                            </ListItem>
                          ))}
                        </List>
                      </VStack>
                    </Card>
                  ))}
                </Grid>
              </VStack>
            </CardBody>
          </Card>
        </Container>
      </Flex>

      {/* Slide 8: SAGE Platform */}
      <Flex
        minH="100vh"
        bgGradient={slides[7].background}
        align="center"
        justify="center"
        position="relative"
        py={20}
      >
        <Container maxW="1400px">
          <Card bg="white" borderRadius="3xl" p={12} boxShadow="2xl">
            <CardBody>
              <VStack spacing={8}>
                <VStack spacing={2}>
                  <Heading size="3xl" color="purple.700" textAlign="center">
                    SAGE
                  </Heading>
                  <Heading size="lg" color="gray.600" textAlign="center">
                    Universal Development Intelligence Platform
                  </Heading>
                  <Text fontSize="lg" textAlign="center" color="gray.600">
                    Orchestrating 100+ Specialized AI Experts from Silicon to Solution
                  </Text>
                </VStack>
                
                <Grid templateColumns={{ base: "1fr", md: "repeat(2, 1fr)", lg: "repeat(3, 1fr)" }} gap={6}>
                  {[
                    {
                      emoji: "🎯",
                      title: "Complete Stack Mastery",
                      points: [
                        "Nix Expert: Kernel to cloud expertise",
                        "Resource Expert: Real-world aware",
                        "Domain & DDD Experts",
                        "Infrastructure Experts"
                      ]
                    },
                    {
                      emoji: "🔬",
                      title: "Mathematical Foundation",
                      points: [
                        "Provable Correctness",
                        "CID Chain Coherence",
                        "Verified State Transitions",
                        "Algebraic Message Routing"
                      ]
                    },
                    {
                      emoji: "🧠",
                      title: "Distributed Intelligence",
                      points: [
                        "Zero Single Points of Failure",
                        "Auto-Healing Architecture",
                        "Multi-Agent Orchestration",
                        "Event-Sourced Truth"
                      ]
                    },
                    {
                      emoji: "🚀",
                      title: "Development Acceleration",
                      points: [
                        "Proactive Guidance",
                        "Living Documentation",
                        "Compliance by Design",
                        "Context Preservation"
                      ]
                    },
                    {
                      emoji: "💡",
                      title: "Key Capabilities",
                      points: [
                        "Universal LLM Adapter",
                        "Hot-Swappable Agents",
                        "TEA-ECS Bridge",
                        "Graph Architecture"
                      ]
                    },
                    {
                      emoji: "🎯",
                      title: "Real-World Impact",
                      points: [
                        "No More 'Works on My Machine'",
                        "Resource-Optimal Solutions",
                        "Domain Certainty",
                        "Audit-Ready Systems"
                      ]
                    }
                  ].map((feature, idx) => (
                    <Card key={idx} borderRadius="xl" overflow="hidden" boxShadow="lg">
                      <Box
                        h="120px"
                        bgGradient="linear(135deg, #667eea 0%, #764ba2 100%)"
                        display="flex"
                        alignItems="center"
                        justifyContent="center"
                        fontSize="4xl"
                      >
                        {feature.emoji}
                      </Box>
                      <CardBody p={6}>
                        <Heading size="md" mb={4}>{feature.title}</Heading>
                        <List spacing={2}>
                          {feature.points.map((point, i) => (
                            <ListItem key={i} fontSize="sm" color="gray.600">
                              • {point}
                            </ListItem>
                          ))}
                        </List>
                      </CardBody>
                    </Card>
                  ))}
                </Grid>

                <Box bg="purple.50" p={8} borderRadius="xl" w="full" mt={8}>
                  <VStack spacing={4}>
                    <Heading size="lg" color="purple.700">🔮 The SAGE Difference</Heading>
                    <List spacing={2}>
                      <ListItem>• 100+ Specialized AI Agent Experts scaling to billions</ListItem>
                      <ListItem>• Mathematical Proofs not promises</ListItem>
                      <ListItem>• Distributed Resilience not single points of failure</ListItem>
                      <ListItem>• Orchestrated Intelligence not isolated tools</ListItem>
                      <ListItem>• Domain Mastery not generic solutions</ListItem>
                    </List>
                    <Text fontStyle="italic" fontSize="lg" textAlign="center" mt={4}>
                      Transform your development with orchestrated intelligence.
                      <br />
                      <strong>Deploy with confidence. Scale with certainty. Evolve with intelligence.</strong>
                    </Text>
                  </VStack>
                </Box>
              </VStack>
            </CardBody>
          </Card>
        </Container>
      </Flex>
    </Box>
  )
}