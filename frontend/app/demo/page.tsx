'use client';

import Link from "next/link";
import { motion } from "framer-motion";
import { Button } from "@/components/ui/button";
import { GlassCard } from "@/components/ui/glass-card";
import { BentoGrid, BentoCard } from "@/components/ui/bento-grid";
import { Navbar } from "@/components/layout/Navbar";
import { Footer } from "@/components/layout/Footer";
import {
  Play,
  Calendar,
  CheckCircle,
  Activity,
  BarChart3,
  MessageSquare,
  Users,
  FileText,
  Zap,
  Shield
} from "lucide-react";
import { useState } from "react";

export default function DemoPage() {
  const [selectedFeature, setSelectedFeature] = useState(0);

  const features = [
    {
      id: 0,
      title: "Patient Management",
      description: "Register patients with Aadhaar verification and ABDM Health ID in seconds",
      icon: Users,
      color: "blue",
      highlights: [
        "Aadhaar verification",
        "ABDM Health ID creation",
        "Multi-language support",
        "Consent management",
      ],
    },
    {
      id: 1,
      title: "Sample Tracking",
      description: "Real-time barcode-based sample tracking from collection to disposal",
      icon: Activity,
      color: "green",
      highlights: [
        "Auto barcode generation",
        "Real-time status updates",
        "Chain of custody",
        "Temperature monitoring",
      ],
    },
    {
      id: 2,
      title: "AI Auto-Verification",
      description: "Intelligent result verification with 30-60% automation rate",
      icon: Zap,
      color: "purple",
      highlights: [
        "ML-powered validation",
        "Delta check analysis",
        "Westgard rules",
        "Critical value alerts",
      ],
    },
    {
      id: 3,
      title: "Reports & Analytics",
      description: "Comprehensive dashboards and customizable reports",
      icon: BarChart3,
      color: "orange",
      highlights: [
        "Real-time dashboards",
        "Custom report builder",
        "Revenue analytics",
        "Performance metrics",
      ],
    },
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
            className="text-center max-w-4xl mx-auto space-y-6"
          >
            <div className="inline-block px-4 py-2 rounded-full bg-primary/10 text-primary text-sm font-medium">
              Product Demo
            </div>

            <h1 className="text-5xl md:text-6xl font-bold">
              See Floe-LIS in <span className="gradient-text">Action</span>
            </h1>

            <p className="text-xl text-slate-600 leading-relaxed">
              Discover how Floe-LIS transforms laboratory workflows with AI-powered automation,
              real-time tracking, and seamless integrations.
            </p>

            <div className="flex flex-col sm:flex-row gap-4 justify-center pt-4">
              <Button size="lg" className="hover-glow">
                <Calendar className="w-5 h-5 mr-2" />
                Schedule Live Demo
              </Button>
              <Button size="lg" variant="outline" className="hover-lift">
                <Play className="w-5 h-5 mr-2" />
                Watch Video Tour
              </Button>
            </div>
          </motion.div>
        </div>
      </section>

      {/* Interactive Feature Showcase */}
      <section className="section-padding">
        <div className="container-wide">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="text-center mb-12"
          >
            <h2 className="text-4xl font-bold mb-4">
              Explore Key <span className="gradient-text">Features</span>
            </h2>
            <p className="text-xl text-slate-600">
              Click on any feature to see it in action
            </p>
          </motion.div>

          <div className="grid lg:grid-cols-2 gap-12 items-center">
            {/* Feature Tabs */}
            <div className="space-y-4">
              {features.map((feature) => (
                <motion.div
                  key={feature.id}
                  initial={{ opacity: 0, x: -20 }}
                  whileInView={{ opacity: 1, x: 0 }}
                  viewport={{ once: true }}
                  transition={{ delay: feature.id * 0.1 }}
                >
                  <GlassCard
                    className={`p-6 cursor-pointer transition-all ${
                      selectedFeature === feature.id
                        ? 'ring-2 ring-primary shadow-xl'
                        : 'hover:shadow-lg'
                    }`}
                    onClick={() => setSelectedFeature(feature.id)}
                  >
                    <div className="flex items-start gap-4">
                      <div className={`p-3 rounded-xl bg-${feature.color}-500/10 flex-shrink-0`}>
                        <feature.icon className={`w-6 h-6 text-${feature.color}-600`} />
                      </div>
                      <div className="flex-1">
                        <h3 className="text-xl font-bold mb-2">{feature.title}</h3>
                        <p className="text-slate-600 text-sm mb-4">{feature.description}</p>
                        {selectedFeature === feature.id && (
                          <ul className="space-y-2">
                            {feature.highlights.map((highlight, i) => (
                              <li key={i} className="flex items-center gap-2 text-sm text-slate-700">
                                <CheckCircle className="w-4 h-4 text-primary" />
                                {highlight}
                              </li>
                            ))}
                          </ul>
                        )}
                      </div>
                    </div>
                  </GlassCard>
                </motion.div>
              ))}
            </div>

            {/* Feature Preview */}
            <motion.div
              key={selectedFeature}
              initial={{ opacity: 0, scale: 0.95 }}
              animate={{ opacity: 1, scale: 1 }}
              transition={{ duration: 0.3 }}
            >
              <GlassCard variant="strong" className="p-8 aspect-video flex items-center justify-center">
                <div className="text-center space-y-4">
                  <div className={`p-6 rounded-2xl bg-${features[selectedFeature].color}-500/10 w-fit mx-auto`}>
                    {(() => {
                      const Icon = features[selectedFeature].icon;
                      return <Icon className={`w-16 h-16 text-${features[selectedFeature].color}-600`} />;
                    })()}
                  </div>
                  <h3 className="text-2xl font-bold">{features[selectedFeature].title}</h3>
                  <p className="text-slate-600">
                    Interactive demo coming soon. Schedule a live demo to see it in action.
                  </p>
                  <Button className="hover-glow">
                    Schedule Demo
                  </Button>
                </div>
              </GlassCard>
            </motion.div>
          </div>
        </div>
      </section>

      {/* Workflow Demo */}
      <section className="section-padding bg-slate-50">
        <div className="container-wide">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl font-bold mb-4">
              Complete <span className="gradient-text">Workflow</span>
            </h2>
            <p className="text-xl text-slate-600">
              From patient registration to report delivery in minutes
            </p>
          </motion.div>

          <BentoGrid>
            <BentoCard size="medium" index={0}>
              <div className="space-y-4">
                <div className="p-3 rounded-xl bg-blue-500/10 w-fit">
                  <Users className="w-8 h-8 text-blue-600" />
                </div>
                <div className="text-4xl font-bold text-blue-600">01</div>
                <h3 className="text-xl font-bold">Patient Registration</h3>
                <p className="text-slate-600 text-sm">
                  Quick registration with Aadhaar verification and ABDM Health ID creation
                </p>
              </div>
            </BentoCard>

            <BentoCard size="medium" index={1}>
              <div className="space-y-4">
                <div className="p-3 rounded-xl bg-green-500/10 w-fit">
                  <Activity className="w-8 h-8 text-green-600" />
                </div>
                <div className="text-4xl font-bold text-green-600">02</div>
                <h3 className="text-xl font-bold">Sample Collection</h3>
                <p className="text-slate-600 text-sm">
                  Barcode generation and real-time tracking
                </p>
              </div>
            </BentoCard>

            <BentoCard size="medium" index={2}>
              <div className="space-y-4">
                <div className="p-3 rounded-xl bg-purple-500/10 w-fit">
                  <Zap className="w-8 h-8 text-purple-600" />
                </div>
                <div className="text-4xl font-bold text-purple-600">03</div>
                <h3 className="text-xl font-bold">AI Verification</h3>
                <p className="text-slate-600 text-sm">
                  Automated result verification with ML
                </p>
              </div>
            </BentoCard>

            <BentoCard size="large" index={3}>
              <div className="space-y-4">
                <div className="p-3 rounded-xl bg-orange-500/10 w-fit">
                  <FileText className="w-8 h-8 text-orange-600" />
                </div>
                <div className="text-4xl font-bold text-orange-600">04</div>
                <h3 className="text-xl font-bold">Report Delivery</h3>
                <p className="text-slate-600 text-sm mb-3">
                  Multi-channel delivery via WhatsApp, Email, and Patient Portal
                </p>
                <div className="flex gap-2">
                  <span className="px-3 py-1 rounded-full bg-orange-500/10 text-orange-600 text-xs font-medium">
                    WhatsApp
                  </span>
                  <span className="px-3 py-1 rounded-full bg-orange-500/10 text-orange-600 text-xs font-medium">
                    Email
                  </span>
                  <span className="px-3 py-1 rounded-full bg-orange-500/10 text-orange-600 text-xs font-medium">
                    Portal
                  </span>
                </div>
              </div>
            </BentoCard>

            <BentoCard size="medium" index={4}>
              <div className="space-y-4">
                <div className="p-3 rounded-xl bg-red-500/10 w-fit">
                  <Shield className="w-8 h-8 text-red-600" />
                </div>
                <div className="text-4xl font-bold text-red-600">05</div>
                <h3 className="text-xl font-bold">Compliance</h3>
                <p className="text-slate-600 text-sm">
                  Automatic NABL compliance and audit trails
                </p>
              </div>
            </BentoCard>

            <BentoCard size="medium" index={5}>
              <div className="space-y-4">
                <div className="p-3 rounded-xl bg-primary/10 w-fit">
                  <BarChart3 className="w-8 h-8 text-primary" />
                </div>
                <div className="text-4xl font-bold text-primary">06</div>
                <h3 className="text-xl font-bold">Analytics</h3>
                <p className="text-slate-600 text-sm">
                  Real-time insights and performance metrics
                </p>
              </div>
            </BentoCard>
          </BentoGrid>
        </div>
      </section>

      {/* Video/Screenshot Section */}
      <section className="section-padding">
        <div className="container-wide">
          <GlassCard variant="strong" className="p-8">
            <div className="aspect-video bg-slate-100 rounded-xl flex items-center justify-center">
              <div className="text-center space-y-4">
                <div className="p-6 rounded-full bg-primary/10 w-fit mx-auto">
                  <Play className="w-12 h-12 text-primary" />
                </div>
                <p className="text-slate-600">
                  Product demo video coming soon
                </p>
              </div>
            </div>
          </GlassCard>
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
                Ready to Transform Your Lab?
              </h2>

              <p className="text-xl text-slate-600">
                Schedule a personalized demo with our team and see how Floe-LIS can streamline your operations.
              </p>

              <div className="flex flex-col sm:flex-row gap-4 justify-center pt-6">
                <Button size="lg" className="hover-glow">
                  <Calendar className="w-5 h-5 mr-2" />
                  Schedule Live Demo
                </Button>
                <Link href="/register">
                  <Button size="lg" variant="outline" className="hover-lift">
                    Start Free Trial
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
