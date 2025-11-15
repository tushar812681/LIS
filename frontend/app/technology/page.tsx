'use client';

import Link from "next/link";
import { motion } from "framer-motion";
import { Button } from "@/components/ui/button";
import { GlassCard } from "@/components/ui/glass-card";
import { Navbar } from "@/components/layout/Navbar";
import { Footer } from "@/components/layout/Footer";
import {
  Code,
  Server,
  Database,
  Cloud,
  Shield,
  Zap,
  Layers,
  Globe,
  Lock,
  GitBranch,
  Container,
  Smartphone,
  Check,
  TrendingUp
} from "lucide-react";

export default function TechnologyPage() {
  const techStack = [
    {
      category: "Backend",
      description: "High-performance, secure, and scalable backend infrastructure",
      technologies: [
        {
          name: "Rust",
          icon: Server,
          description: "Memory-safe, blazingly fast backend services",
          benefits: [
            "Zero-cost abstractions",
            "Memory safety without GC",
            "Fearless concurrency",
            "3x faster than Node.js"
          ],
          color: "orange"
        },
        {
          name: "GraphQL",
          icon: GitBranch,
          description: "Flexible API layer for efficient data fetching",
          benefits: [
            "Type-safe queries",
            "Real-time subscriptions",
            "Optimized data loading",
            "Self-documenting API"
          ],
          color: "pink"
        },
        {
          name: "PostgreSQL",
          icon: Database,
          description: "Robust relational database with advanced features",
          benefits: [
            "ACID compliance",
            "JSON support",
            "Full-text search",
            "Proven reliability"
          ],
          color: "blue"
        }
      ]
    },
    {
      category: "Frontend",
      description: "Modern, responsive, and performant user interfaces",
      technologies: [
        {
          name: "Next.js 16",
          icon: Code,
          description: "React framework with server-side rendering and static generation",
          benefits: [
            "App Router architecture",
            "Server components",
            "Built-in optimization",
            "Edge runtime support"
          ],
          color: "primary"
        },
        {
          name: "TypeScript",
          icon: Code,
          description: "Type-safe development with enhanced IDE support",
          benefits: [
            "Compile-time error checking",
            "Enhanced autocomplete",
            "Better refactoring",
            "Self-documenting code"
          ],
          color: "blue"
        },
        {
          name: "Tailwind CSS",
          icon: Layers,
          description: "Utility-first CSS framework for rapid UI development",
          benefits: [
            "Consistent design system",
            "Minimal CSS bundle",
            "Dark mode support",
            "Responsive by default"
          ],
          color: "cyan"
        }
      ]
    },
    {
      category: "Infrastructure",
      description: "Cloud-native architecture for reliability and scale",
      technologies: [
        {
          name: "Kubernetes",
          icon: Container,
          description: "Container orchestration for seamless deployments",
          benefits: [
            "Auto-scaling",
            "Self-healing",
            "Rolling updates",
            "Multi-region support"
          ],
          color: "blue"
        },
        {
          name: "AWS",
          icon: Cloud,
          description: "Enterprise-grade cloud infrastructure",
          benefits: [
            "99.99% availability",
            "Global CDN",
            "Managed services",
            "Cost optimization"
          ],
          color: "orange"
        },
        {
          name: "Redis",
          icon: Zap,
          description: "In-memory caching for blazing fast performance",
          benefits: [
            "Sub-millisecond latency",
            "Session management",
            "Real-time analytics",
            "Pub/sub messaging"
          ],
          color: "red"
        }
      ]
    },
    {
      category: "Security",
      description: "Enterprise-grade security and compliance",
      technologies: [
        {
          name: "End-to-End Encryption",
          icon: Lock,
          description: "AES-256 encryption for data at rest and in transit",
          benefits: [
            "TLS 1.3 protocol",
            "Encrypted backups",
            "HSM key management",
            "Zero-knowledge architecture"
          ],
          color: "purple"
        },
        {
          name: "OAuth 2.0 + JWT",
          icon: Shield,
          description: "Industry-standard authentication and authorization",
          benefits: [
            "Secure token-based auth",
            "Role-based access",
            "SSO support",
            "Multi-factor auth"
          ],
          color: "green"
        },
        {
          name: "SOC 2 Compliance",
          icon: Shield,
          description: "Comprehensive security and compliance framework",
          benefits: [
            "Regular audits",
            "Penetration testing",
            "DPDP 2023 compliant",
            "HIPAA ready"
          ],
          color: "red"
        }
      ]
    },
    {
      category: "DevOps & Monitoring",
      description: "Continuous delivery and observability",
      technologies: [
        {
          name: "CI/CD Pipeline",
          icon: GitBranch,
          description: "Automated testing and deployment",
          benefits: [
            "Automated testing",
            "Blue-green deployments",
            "Rollback capability",
            "Multiple environments"
          ],
          color: "primary"
        },
        {
          name: "Observability",
          icon: TrendingUp,
          description: "Comprehensive monitoring and alerting",
          benefits: [
            "Real-time metrics",
            "Distributed tracing",
            "Log aggregation",
            "Custom dashboards"
          ],
          color: "orange"
        },
        {
          name: "Backup & DR",
          icon: Database,
          description: "Automated backup and disaster recovery",
          benefits: [
            "Hourly backups",
            "Point-in-time recovery",
            "Multi-region replication",
            "RTO < 1 hour"
          ],
          color: "blue"
        }
      ]
    },
    {
      category: "Integration",
      description: "Seamless connectivity with external systems",
      technologies: [
        {
          name: "HL7 & FHIR",
          icon: Globe,
          description: "Healthcare interoperability standards",
          benefits: [
            "HL7 v2.x support",
            "FHIR R4 implementation",
            "ABDM integration",
            "Bidirectional sync"
          ],
          color: "green"
        },
        {
          name: "REST & WebSocket",
          icon: Zap,
          description: "Modern API protocols for real-time communication",
          benefits: [
            "RESTful APIs",
            "WebSocket support",
            "Server-sent events",
            "Rate limiting"
          ],
          color: "purple"
        },
        {
          name: "WhatsApp Business API",
          icon: Smartphone,
          description: "Native integration for patient communication",
          benefits: [
            "Official API partner",
            "Rich media support",
            "Template messages",
            "Delivery tracking"
          ],
          color: "green"
        }
      ]
    }
  ];

  const architectureHighlights = [
    {
      icon: Layers,
      title: "Microservices Architecture",
      description: "Independently deployable services for better scalability and maintainability",
      color: "blue"
    },
    {
      icon: Zap,
      title: "Event-Driven Design",
      description: "Asynchronous processing for improved responsiveness and reliability",
      color: "purple"
    },
    {
      icon: Shield,
      title: "Zero-Trust Security",
      description: "Every request authenticated and authorized, no implicit trust",
      color: "red"
    },
    {
      icon: Globe,
      title: "Multi-Tenant by Design",
      description: "Secure data isolation with shared infrastructure efficiency",
      color: "green"
    }
  ];

  return (
    <div className="min-h-screen bg-white">
      <Navbar />

      {/* Hero Section */}
      <section className="relative overflow-hidden">
        <div className="absolute inset-0 gradient-mesh opacity-40" />
        <div className="absolute inset-0 grid-pattern" />

        <div className="container-wide section-padding relative">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="text-center max-w-3xl mx-auto space-y-6"
          >
            <div className="inline-block px-4 py-2 rounded-full bg-primary/10 text-primary text-sm font-medium">
              Enterprise-Grade Technology
            </div>

            <h1 className="text-5xl md:text-6xl font-bold">
              Built with <span className="gradient-text">Modern Tech</span>
            </h1>

            <p className="text-xl text-slate-600">
              Cutting-edge technology stack designed for performance, security, and scalability
            </p>
          </motion.div>
        </div>
      </section>

      {/* Quick Tech Stack Overview */}
      <section className="py-12 bg-slate-50">
        <div className="container-wide">
          <div className="grid grid-cols-2 md:grid-cols-4 gap-6">
            {[
              { name: "Rust", desc: "Backend", icon: Server },
              { name: "GraphQL", desc: "API Layer", icon: GitBranch },
              { name: "Next.js", desc: "Frontend", icon: Code },
              { name: "Kubernetes", desc: "Deployment", icon: Container },
            ].map((tech, i) => (
              <motion.div
                key={i}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: i * 0.1 }}
              >
                <GlassCard className="p-6 text-center hover-lift">
                  <tech.icon className="w-12 h-12 text-primary mx-auto mb-4" />
                  <div className="text-2xl font-bold text-primary mb-2">{tech.name}</div>
                  <div className="text-sm text-slate-600">{tech.desc}</div>
                </GlassCard>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Architecture Highlights */}
      <section className="section-padding">
        <div className="container-wide">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="text-center mb-12"
          >
            <h2 className="text-4xl font-bold mb-4">Architecture Principles</h2>
            <p className="text-xl text-slate-600 max-w-2xl mx-auto">
              Built on proven architectural patterns for enterprise applications
            </p>
          </motion.div>

          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
            {architectureHighlights.map((highlight, i) => (
              <motion.div
                key={i}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ delay: i * 0.1 }}
              >
                <GlassCard className="p-6 h-full hover-lift text-center">
                  <div className={`p-3 rounded-xl bg-${highlight.color}-500/10 w-fit mx-auto mb-4`}>
                    <highlight.icon className={`w-8 h-8 text-${highlight.color}-600`} />
                  </div>
                  <h3 className="font-bold text-lg mb-3">{highlight.title}</h3>
                  <p className="text-sm text-slate-600">{highlight.description}</p>
                </GlassCard>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Detailed Technology Stack */}
      {techStack.map((stack, stackIndex) => (
        <section
          key={stackIndex}
          className={`section-padding ${stackIndex % 2 === 0 ? "bg-slate-50" : "bg-white"}`}
        >
          <div className="container-wide">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              className="text-center mb-12"
            >
              <h2 className="text-4xl font-bold mb-4">{stack.category}</h2>
              <p className="text-xl text-slate-600 max-w-2xl mx-auto">
                {stack.description}
              </p>
            </motion.div>

            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
              {stack.technologies.map((tech, techIndex) => (
                <motion.div
                  key={techIndex}
                  initial={{ opacity: 0, y: 20 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  viewport={{ once: true }}
                  transition={{ delay: techIndex * 0.1 }}
                >
                  <GlassCard className="p-8 h-full hover-lift">
                    <div className={`p-3 rounded-xl bg-${tech.color}-500/10 w-fit mb-4`}>
                      <tech.icon className={`w-8 h-8 text-${tech.color}-600`} />
                    </div>
                    <h3 className="text-2xl font-bold mb-3">{tech.name}</h3>
                    <p className="text-slate-600 mb-6">{tech.description}</p>
                    <ul className="space-y-3">
                      {tech.benefits.map((benefit, benefitIndex) => (
                        <li key={benefitIndex} className="flex items-start gap-2">
                          <Check className={`w-5 h-5 text-${tech.color}-600 flex-shrink-0 mt-0.5`} />
                          <span className="text-sm text-slate-600">{benefit}</span>
                        </li>
                      ))}
                    </ul>
                  </GlassCard>
                </motion.div>
              ))}
            </div>
          </div>
        </section>
      ))}

      {/* Performance Metrics */}
      <section className="section-padding bg-slate-50">
        <div className="container-wide">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="text-center mb-12"
          >
            <h2 className="text-4xl font-bold mb-4">Performance Metrics</h2>
            <p className="text-xl text-slate-600 max-w-2xl mx-auto">
              Industry-leading performance across all metrics
            </p>
          </motion.div>

          <div className="grid md:grid-cols-4 gap-6">
            {[
              { metric: "99.9%", label: "Uptime SLA", icon: TrendingUp },
              { metric: "<100ms", label: "API Response", icon: Zap },
              { metric: "10K+", label: "Requests/sec", icon: Server },
              { metric: "256-bit", label: "AES Encryption", icon: Lock },
            ].map((stat, i) => (
              <motion.div
                key={i}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ delay: i * 0.1 }}
              >
                <GlassCard className="p-8 text-center">
                  <stat.icon className="w-10 h-10 text-primary mx-auto mb-4" />
                  <div className="text-4xl font-bold gradient-text mb-2">{stat.metric}</div>
                  <div className="text-sm text-slate-600">{stat.label}</div>
                </GlassCard>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Why Our Tech Stack */}
      <section className="section-padding">
        <div className="container-wide">
          <div className="grid lg:grid-cols-2 gap-12 items-center">
            <motion.div
              initial={{ opacity: 0, x: -20 }}
              whileInView={{ opacity: 1, x: 0 }}
              viewport={{ once: true }}
            >
              <h2 className="text-4xl font-bold mb-6">
                Why This <span className="gradient-text">Tech Stack</span>?
              </h2>
              <div className="space-y-6">
                <div>
                  <h3 className="text-xl font-bold mb-2">Performance First</h3>
                  <p className="text-slate-600">
                    Rust and Next.js provide industry-leading performance, ensuring your laboratory
                    operations run smoothly even during peak hours.
                  </p>
                </div>
                <div>
                  <h3 className="text-xl font-bold mb-2">Security by Design</h3>
                  <p className="text-slate-600">
                    Memory-safe Rust backend, end-to-end encryption, and zero-trust architecture
                    ensure your data is always protected.
                  </p>
                </div>
                <div>
                  <h3 className="text-xl font-bold mb-2">Scale with Confidence</h3>
                  <p className="text-slate-600">
                    Cloud-native architecture on Kubernetes allows automatic scaling to handle
                    your growth without infrastructure worries.
                  </p>
                </div>
                <div>
                  <h3 className="text-xl font-bold mb-2">Future-Proof</h3>
                  <p className="text-slate-600">
                    Modern technologies and standards ensure Floe-LIS stays current with evolving
                    healthcare and technology landscapes.
                  </p>
                </div>
              </div>
            </motion.div>

            <motion.div
              initial={{ opacity: 0, x: 20 }}
              whileInView={{ opacity: 1, x: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard variant="strong" className="p-8">
                <h3 className="text-2xl font-bold mb-6">Technology Benefits</h3>
                <ul className="space-y-4">
                  {[
                    "3x faster than traditional Node.js applications",
                    "99.9% uptime with automatic failover",
                    "Sub-100ms API response times",
                    "Scales automatically from 1 to 10,000+ users",
                    "Zero-downtime deployments and updates",
                    "Enterprise-grade security and compliance",
                    "Real-time data synchronization",
                    "Mobile-first responsive design"
                  ].map((benefit, i) => (
                    <li key={i} className="flex items-start gap-3">
                      <Check className="w-5 h-5 text-primary flex-shrink-0 mt-0.5" />
                      <span className="text-slate-600">{benefit}</span>
                    </li>
                  ))}
                </ul>
              </GlassCard>
            </motion.div>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="section-padding bg-slate-50">
        <div className="container-wide">
          <GlassCard variant="strong" className="p-12 md:p-16 text-center">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              className="space-y-6 max-w-2xl mx-auto"
            >
              <h2 className="text-4xl font-bold">
                Experience the Technology
              </h2>

              <p className="text-xl text-slate-600">
                See how our modern tech stack delivers superior performance and reliability
              </p>

              <div className="flex flex-col sm:flex-row gap-4 justify-center">
                <Link href="/demo">
                  <Button size="lg" className="w-full sm:w-auto hover-glow">
                    Schedule a Demo
                  </Button>
                </Link>
                <Link href="/contact">
                  <Button size="lg" variant="outline" className="w-full sm:w-auto hover-lift">
                    Talk to an Engineer
                  </Button>
                </Link>
              </div>
            </motion.div>
          </GlassCard>
        </div>
      </section>

      <Footer />
    </div>
  );
}
