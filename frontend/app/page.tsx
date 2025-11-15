'use client';

import Link from "next/link";
import { motion } from "framer-motion";
import { Button } from "@/components/ui/button";
import { BentoGrid, BentoCard } from "@/components/ui/bento-grid";
import { GlassCard } from "@/components/ui/glass-card";
import { AnimatedCounter } from "@/components/ui/animated-counter";
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
  Award,
  Clock,
  TrendingUp
} from "lucide-react";

export default function Home() {
  return (
    <div className="min-h-screen bg-white">
      <Navbar />

      {/* Hero Section - Modern Split Screen */}
      <section className="relative overflow-hidden">
        {/* Mesh Gradient Background */}
        <div className="absolute inset-0 gradient-mesh opacity-60" />
        <div className="absolute inset-0 grid-pattern" />

        <div className="container-wide section-padding relative">
          <div className="grid lg:grid-cols-2 gap-12 items-center">
            {/* Left Content */}
            <motion.div
              initial={{ opacity: 0, x: -50 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ duration: 0.8 }}
              className="space-y-8"
            >
              <div className="space-y-4">
                <motion.div
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: 0.2 }}
                  className="inline-block"
                >
                  <span className="px-4 py-2 rounded-full bg-primary/10 text-primary text-sm font-medium">
                    Trusted by 500+ Labs across India
                  </span>
                </motion.div>

                <h1 className="text-5xl md:text-6xl lg:text-7xl font-bold tracking-tight">
                  Modern{" "}
                  <span className="gradient-text">
                    Laboratory
                  </span>
                  <br />
                  Information System
                </h1>

                <p className="text-xl text-slate-600 max-w-2xl leading-relaxed">
                  Cloud-native LIS/LIMS platform designed for Indian healthcare market with
                  ABDM integration, WhatsApp notifications, and AI-powered automation.
                </p>
              </div>

              <div className="flex flex-col sm:flex-row gap-4">
                <Link href="/patients/register">
                  <Button size="lg" className="w-full sm:w-auto text-lg px-8 hover-glow">
                    Register Patient
                  </Button>
                </Link>
                <Link href="/demo">
                  <Button size="lg" variant="outline" className="w-full sm:w-auto text-lg px-8 hover-lift">
                    View Demo
                  </Button>
                </Link>
              </div>

              {/* Quick Stats */}
              <div className="grid grid-cols-3 gap-6 pt-8">
                {[
                  { label: "Labs", value: 500, suffix: "+" },
                  { label: "Tests/Month", value: 100, suffix: "K+" },
                  { label: "Automation", value: 60, suffix: "%" },
                ].map((stat, i) => (
                  <div key={i} className="space-y-1">
                    <AnimatedCounter
                      value={stat.value}
                      suffix={stat.suffix}
                      className="text-3xl font-bold text-primary"
                    />
                    <p className="text-sm text-slate-600">{stat.label}</p>
                  </div>
                ))}
              </div>
            </motion.div>

            {/* Right Visual */}
            <motion.div
              initial={{ opacity: 0, x: 50 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ duration: 0.8, delay: 0.2 }}
              className="relative hidden lg:block"
            >
              <div className="relative">
                {/* Floating Cards */}
                <GlassCard className="p-6 absolute -top-4 -left-4 w-64">
                  <div className="flex items-center gap-3">
                    <div className="p-3 rounded-full bg-primary/10">
                      <Activity className="w-6 h-6 text-primary" />
                    </div>
                    <div>
                      <p className="text-sm font-medium">Sample Tracked</p>
                      <p className="text-xs text-slate-600">Real-time status</p>
                    </div>
                  </div>
                </GlassCard>

                <GlassCard className="p-6 absolute top-20 -right-4 w-64">
                  <div className="flex items-center gap-3">
                    <div className="p-3 rounded-full bg-green-500/10">
                      <Check className="w-6 h-6 text-green-600" />
                    </div>
                    <div>
                      <p className="text-sm font-medium">AI Verified</p>
                      <p className="text-xs text-slate-600">30-60% automation</p>
                    </div>
                  </div>
                </GlassCard>

                <GlassCard className="p-6 absolute bottom-4 left-12 w-64">
                  <div className="flex items-center gap-3">
                    <div className="p-3 rounded-full bg-blue-500/10">
                      <Shield className="w-6 h-6 text-blue-600" />
                    </div>
                    <div>
                      <p className="text-sm font-medium">NABL Compliant</p>
                      <p className="text-xs text-slate-600">ISO 15189:2022</p>
                    </div>
                  </div>
                </GlassCard>

                {/* Central Mesh Gradient */}
                <div className="w-full h-96 gradient-mesh rounded-3xl shadow-2xl" />
              </div>
            </motion.div>
          </div>
        </div>
      </section>

      {/* Stats Section */}
      <section className="section-padding bg-slate-50">
        <div className="container-wide">
          <GlassCard variant="strong" className="p-12">
            <div className="grid grid-cols-2 md:grid-cols-4 gap-8">
              {[
                { icon: Users, label: "Active Users", value: 5000, suffix: "+" },
                { icon: Activity, label: "Daily Tests", value: 10000, suffix: "+" },
                { icon: Clock, label: "Avg TAT", value: 4, suffix: "hrs" },
                { icon: TrendingUp, label: "Uptime", value: 99.9, suffix: "%" },
              ].map((stat, i) => (
                <motion.div
                  key={i}
                  initial={{ opacity: 0, y: 20 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  viewport={{ once: true }}
                  transition={{ delay: i * 0.1 }}
                  className="text-center space-y-2"
                >
                  <stat.icon className="w-8 h-8 text-primary mx-auto mb-3" />
                  <AnimatedCounter
                    value={stat.value}
                    suffix={stat.suffix}
                    className="text-4xl font-bold text-slate-900"
                  />
                  <p className="text-sm text-slate-600">{stat.label}</p>
                </motion.div>
              ))}
            </div>
          </GlassCard>
        </div>
      </section>

      {/* Features Section - Bento Grid */}
      <section id="features" className="section-padding">
        <div className="container-wide">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl md:text-5xl font-bold text-slate-900 mb-4">
              Powerful <span className="gradient-text">Features</span>
            </h2>
            <p className="text-xl text-slate-600 max-w-2xl mx-auto">
              Everything you need to run a modern laboratory
            </p>
          </motion.div>

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

      {/* Technology Stack */}
      <section id="technology" className="section-padding bg-slate-50">
        <div className="container-wide">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl md:text-5xl font-bold text-slate-900 mb-4">
              Built with <span className="gradient-text">Modern Tech</span>
            </h2>
            <p className="text-xl text-slate-600">
              Enterprise-grade technology stack
            </p>
          </motion.div>

          <div className="grid grid-cols-2 md:grid-cols-4 gap-6">
            {[
              { name: "Rust", desc: "Backend" },
              { name: "GraphQL", desc: "API Layer" },
              { name: "Next.js", desc: "Frontend" },
              { name: "Kubernetes", desc: "Deployment" },
            ].map((tech, i) => (
              <GlassCard key={i} className="p-6 text-center">
                <div className="text-2xl font-bold text-primary mb-2">{tech.name}</div>
                <div className="text-sm text-slate-600">{tech.desc}</div>
              </GlassCard>
            ))}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="section-padding relative overflow-hidden">
        <div className="absolute inset-0 gradient-mesh opacity-40" />

        <div className="container-wide relative">
          <GlassCard variant="strong" className="p-12 md:p-16 text-center">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              className="space-y-6"
            >
              <div className="inline-block px-4 py-2 rounded-full bg-primary/10 text-primary text-sm font-medium mb-4">
                <Award className="w-4 h-4 inline mr-2" />
                Join 500+ laboratories
              </div>

              <h2 className="text-4xl md:text-5xl font-bold text-slate-900">
                Ready to Get Started?
              </h2>

              <p className="text-xl text-slate-600 max-w-2xl mx-auto">
                Transform your laboratory with modern, AI-powered automation
              </p>

              <div className="flex flex-col sm:flex-row gap-4 justify-center pt-6">
                <Link href="/register">
                  <Button size="lg" className="w-full sm:w-auto text-lg px-8 hover-glow">
                    Start Free Trial
                  </Button>
                </Link>
                <Link href="/contact">
                  <Button size="lg" variant="outline" className="w-full sm:w-auto text-lg px-8 hover-lift">
                    Contact Sales
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
