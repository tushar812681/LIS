'use client';

import Link from "next/link";
import { motion } from "framer-motion";
import { Button } from "@/components/ui/button";
import { BentoGrid, BentoCard } from "@/components/ui/bento-grid";
import { GlassCard } from "@/components/ui/glass-card";
import { Navbar } from "@/components/layout/Navbar";
import { Footer } from "@/components/layout/Footer";
import {
  Activity,
  Shield,
  Zap,
  Users,
  BarChart3,
  MessageSquare,
  Check,
  Clock,
  FileText,
  Globe,
  Bell,
  Cloud,
  Lock,
  Settings,
  Smartphone,
  TrendingUp
} from "lucide-react";

export default function FeaturesPage() {
  const featureCategories = [
    {
      title: "Core Features",
      description: "Essential functionality for modern laboratory operations",
      features: [
        {
          icon: Users,
          title: "Patient Management",
          description: "Complete patient registration with Aadhaar verification and ABDM Health ID integration",
          benefits: [
            "Aadhaar-based identity verification",
            "ABDM Health ID integration",
            "Support for 7 Indian languages",
            "DPDP 2023 compliant data handling"
          ],
          color: "primary"
        },
        {
          icon: Activity,
          title: "Sample Tracking",
          description: "Real-time tracking from collection to disposal with complete chain of custody",
          benefits: [
            "Automatic barcode generation",
            "Real-time status updates",
            "Complete chain of custody",
            "Temperature monitoring"
          ],
          color: "blue"
        },
        {
          icon: FileText,
          title: "Test Management",
          description: "Comprehensive test catalog with customizable panels and profiles",
          benefits: [
            "Extensive test library",
            "Custom test profiles",
            "Reference range management",
            "Multi-method support"
          ],
          color: "green"
        }
      ]
    },
    {
      title: "Automation & AI",
      description: "Intelligent automation to reduce manual work and improve accuracy",
      features: [
        {
          icon: Zap,
          title: "AI Auto-Verification",
          description: "Machine learning-powered result verification with anomaly detection",
          benefits: [
            "30-60% automation rate",
            "Delta check validation",
            "Pattern recognition",
            "Continuous learning"
          ],
          color: "purple"
        },
        {
          icon: TrendingUp,
          title: "Smart Analytics",
          description: "AI-powered insights and predictive analytics for better decision making",
          benefits: [
            "Predictive TAT analysis",
            "Revenue forecasting",
            "Equipment failure prediction",
            "Workload optimization"
          ],
          color: "orange"
        }
      ]
    },
    {
      title: "Communication",
      description: "Seamless communication with patients and healthcare providers",
      features: [
        {
          icon: MessageSquare,
          title: "WhatsApp Integration",
          description: "Native WhatsApp Business API for seamless patient communication",
          benefits: [
            "Appointment reminders",
            "Sample collection alerts",
            "Digital report delivery",
            "Payment link sharing"
          ],
          color: "green"
        },
        {
          icon: Bell,
          title: "Multi-Channel Notifications",
          description: "Flexible notification system supporting multiple channels",
          benefits: [
            "SMS notifications",
            "Email alerts",
            "Push notifications",
            "Custom notification rules"
          ],
          color: "blue"
        },
        {
          icon: Smartphone,
          title: "Mobile Access",
          description: "Full-featured mobile access for staff and patients",
          benefits: [
            "Progressive web app",
            "Offline capability",
            "Mobile result viewing",
            "Sample collection app"
          ],
          color: "primary"
        }
      ]
    },
    {
      title: "Compliance & Quality",
      description: "Built-in compliance tools for regulatory requirements",
      features: [
        {
          icon: Shield,
          title: "NABL Compliance",
          description: "Built-in ISO 15189:2022 compliance with comprehensive quality management",
          benefits: [
            "IQC/EQC management",
            "TAT monitoring & alerts",
            "Complete audit trail",
            "Quality documentation"
          ],
          color: "red"
        },
        {
          icon: Lock,
          title: "Data Security",
          description: "Enterprise-grade security and data protection",
          benefits: [
            "DPDP 2023 compliant",
            "End-to-end encryption",
            "Role-based access control",
            "Regular security audits"
          ],
          color: "purple"
        },
        {
          icon: FileText,
          title: "ABDM Integration",
          description: "Seamless integration with Ayushman Bharat Digital Mission",
          benefits: [
            "Health ID verification",
            "Digital report submission",
            "PHR integration",
            "Consent management"
          ],
          color: "green"
        }
      ]
    },
    {
      title: "Analytics & Reporting",
      description: "Comprehensive reporting and business intelligence",
      features: [
        {
          icon: BarChart3,
          title: "Advanced Analytics",
          description: "Comprehensive reporting and real-time dashboards",
          benefits: [
            "Real-time dashboards",
            "Custom report builder",
            "Revenue analytics",
            "Performance metrics"
          ],
          color: "orange"
        },
        {
          icon: TrendingUp,
          title: "Business Intelligence",
          description: "Data-driven insights for strategic decision making",
          benefits: [
            "Trend analysis",
            "Comparative reports",
            "Profitability analysis",
            "Market insights"
          ],
          color: "blue"
        }
      ]
    },
    {
      title: "Integration & Infrastructure",
      description: "Modern architecture for seamless integration and scalability",
      features: [
        {
          icon: Cloud,
          title: "Cloud-Native",
          description: "Built on modern cloud infrastructure for reliability and scale",
          benefits: [
            "99.9% uptime guarantee",
            "Auto-scaling",
            "Disaster recovery",
            "Multi-region support"
          ],
          color: "blue"
        },
        {
          icon: Settings,
          title: "Equipment Integration",
          description: "Seamless integration with laboratory analyzers and instruments",
          benefits: [
            "Bidirectional interfacing",
            "Auto-result import",
            "Quality control sync",
            "50+ instruments supported"
          ],
          color: "purple"
        },
        {
          icon: Globe,
          title: "Multi-Location Support",
          description: "Manage multiple labs and collection centers from single platform",
          benefits: [
            "Centralized management",
            "Cross-location reporting",
            "Resource sharing",
            "Unified patient records"
          ],
          color: "green"
        }
      ]
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
              Comprehensive Platform
            </div>

            <h1 className="text-5xl md:text-6xl font-bold">
              Powerful <span className="gradient-text">Features</span>
            </h1>

            <p className="text-xl text-slate-600">
              Everything you need to run a modern, efficient, and compliant laboratory
            </p>
          </motion.div>
        </div>
      </section>

      {/* Quick Feature Highlights - Bento Grid */}
      <section className="py-12">
        <div className="container-wide">
          <BentoGrid>
            <BentoCard size="large" index={0}>
              <div className="space-y-4">
                <div className="p-3 rounded-xl bg-primary/10 w-fit">
                  <Users className="w-8 h-8 text-primary" />
                </div>
                <h3 className="text-2xl font-bold">Patient Management</h3>
                <p className="text-slate-600">
                  Complete patient registration with Aadhaar verification and ABDM Health ID integration
                </p>
                <ul className="space-y-2">
                  {["Aadhaar-based identity", "ABDM Health ID", "7 languages support", "DPDP 2023 compliant"].map((item, i) => (
                    <li key={i} className="flex items-center gap-2 text-sm text-slate-600">
                      <Check className="w-4 h-4 text-primary" />
                      {item}
                    </li>
                  ))}
                </ul>
              </div>
            </BentoCard>

            <BentoCard index={1}>
              <div className="space-y-4">
                <div className="p-3 rounded-xl bg-blue-500/10 w-fit">
                  <Activity className="w-8 h-8 text-blue-600" />
                </div>
                <h3 className="text-2xl font-bold">Sample Tracking</h3>
                <p className="text-slate-600">
                  Real-time tracking from collection to disposal
                </p>
                <ul className="space-y-2">
                  {["Barcode generation", "Status updates", "Chain of custody"].map((item, i) => (
                    <li key={i} className="flex items-center gap-2 text-sm text-slate-600">
                      <Check className="w-4 h-4 text-blue-600" />
                      {item}
                    </li>
                  ))}
                </ul>
              </div>
            </BentoCard>

            <BentoCard index={2}>
              <div className="space-y-4">
                <div className="p-3 rounded-xl bg-purple-500/10 w-fit">
                  <Zap className="w-8 h-8 text-purple-600" />
                </div>
                <h3 className="text-2xl font-bold">AI Auto-Verification</h3>
                <p className="text-slate-600">
                  ML-powered anomaly detection
                </p>
                <div className="text-3xl font-bold gradient-text">30-60%</div>
                <p className="text-sm text-slate-600">Automation rate</p>
              </div>
            </BentoCard>

            <BentoCard size="large" index={3}>
              <div className="space-y-4">
                <div className="p-3 rounded-xl bg-green-500/10 w-fit">
                  <MessageSquare className="w-8 h-8 text-green-600" />
                </div>
                <h3 className="text-2xl font-bold">WhatsApp Integration</h3>
                <p className="text-slate-600">
                  Native WhatsApp Business API for seamless patient communication
                </p>
                <div className="grid grid-cols-2 gap-4">
                  {["Appointment reminders", "Sample collection alerts", "Result delivery", "Payment links"].map((item, i) => (
                    <div key={i} className="flex items-center gap-2 text-sm text-slate-600">
                      <Check className="w-4 h-4 text-green-600" />
                      {item}
                    </div>
                  ))}
                </div>
              </div>
            </BentoCard>

            <BentoCard index={4}>
              <div className="space-y-4">
                <div className="p-3 rounded-xl bg-red-500/10 w-fit">
                  <Shield className="w-8 h-8 text-red-600" />
                </div>
                <h3 className="text-2xl font-bold">NABL Compliance</h3>
                <p className="text-slate-600">
                  Built-in ISO 15189:2022 compliance
                </p>
                <div className="space-y-1">
                  <div className="text-sm font-medium text-red-600">✓ IQC/EQC</div>
                  <div className="text-sm font-medium text-red-600">✓ TAT Monitoring</div>
                  <div className="text-sm font-medium text-red-600">✓ Audit Trail</div>
                </div>
              </div>
            </BentoCard>

            <BentoCard index={5}>
              <div className="space-y-4">
                <div className="p-3 rounded-xl bg-orange-500/10 w-fit">
                  <BarChart3 className="w-8 h-8 text-orange-600" />
                </div>
                <h3 className="text-2xl font-bold">Analytics</h3>
                <p className="text-slate-600">
                  Comprehensive reporting and dashboards
                </p>
                <div className="space-y-1">
                  <div className="text-sm font-medium text-orange-600">• Real-time dashboards</div>
                  <div className="text-sm font-medium text-orange-600">• Revenue analytics</div>
                </div>
              </div>
            </BentoCard>
          </BentoGrid>
        </div>
      </section>

      {/* Detailed Feature Sections */}
      {featureCategories.map((category, categoryIndex) => (
        <section
          key={categoryIndex}
          className={`section-padding ${categoryIndex % 2 === 0 ? "bg-slate-50" : "bg-white"}`}
        >
          <div className="container-wide">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              className="text-center mb-12"
            >
              <h2 className="text-4xl font-bold mb-4">{category.title}</h2>
              <p className="text-xl text-slate-600 max-w-2xl mx-auto">
                {category.description}
              </p>
            </motion.div>

            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
              {category.features.map((feature, featureIndex) => (
                <motion.div
                  key={featureIndex}
                  initial={{ opacity: 0, y: 20 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  viewport={{ once: true }}
                  transition={{ delay: featureIndex * 0.1 }}
                >
                  <GlassCard className="p-8 h-full hover-lift">
                    <div className={`p-3 rounded-xl bg-${feature.color}-500/10 w-fit mb-4`}>
                      <feature.icon className={`w-8 h-8 text-${feature.color}-600`} />
                    </div>
                    <h3 className="text-2xl font-bold mb-3">{feature.title}</h3>
                    <p className="text-slate-600 mb-6">{feature.description}</p>
                    <ul className="space-y-3">
                      {feature.benefits.map((benefit, benefitIndex) => (
                        <li key={benefitIndex} className="flex items-start gap-2">
                          <Check className={`w-5 h-5 text-${feature.color}-600 flex-shrink-0 mt-0.5`} />
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
                Ready to Experience These Features?
              </h2>

              <p className="text-xl text-slate-600">
                See Floe-LIS in action with a personalized demo from our team.
              </p>

              <div className="flex flex-col sm:flex-row gap-4 justify-center">
                <Link href="/demo">
                  <Button size="lg" className="w-full sm:w-auto hover-glow">
                    Schedule a Demo
                  </Button>
                </Link>
                <Link href="/pricing">
                  <Button size="lg" variant="outline" className="w-full sm:w-auto hover-lift">
                    View Pricing
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
