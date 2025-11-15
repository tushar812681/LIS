'use client';

import Link from "next/link";
import { motion } from "framer-motion";
import { Button } from "@/components/ui/button";
import { GlassCard } from "@/components/ui/glass-card";
import { AnimatedCounter } from "@/components/ui/animated-counter";
import { Navbar } from "@/components/layout/Navbar";
import { Footer } from "@/components/layout/Footer";
import { Target, Eye, Heart, Users, Award, TrendingUp, Shield, Zap } from "lucide-react";

export default function AboutPage() {
  const values = [
    {
      icon: Heart,
      title: "Patient First",
      description: "Every decision we make prioritizes patient care and safety.",
      color: "red",
    },
    {
      icon: Shield,
      title: "Quality & Compliance",
      description: "Unwavering commitment to NABL, ISO standards and data security.",
      color: "blue",
    },
    {
      icon: Zap,
      title: "Innovation",
      description: "Leveraging cutting-edge technology to transform healthcare.",
      color: "yellow",
    },
    {
      icon: Users,
      title: "Collaboration",
      description: "Building strong partnerships with labs and healthcare providers.",
      color: "green",
    },
  ];

  const milestones = [
    { year: "2023", event: "Company Founded", desc: "Started with a vision to modernize Indian labs" },
    { year: "2024", event: "50+ Labs Onboarded", desc: "Rapid adoption across Karnataka and Maharashtra" },
    { year: "2024", event: "ABDM Integration", desc: "First LIS to integrate with Ayushman Bharat Digital Mission" },
    { year: "2025", event: "500+ Labs", desc: "Serving laboratories across 15 states" },
  ];

  const stats = [
    { label: "Active Laboratories", value: 500, suffix: "+" },
    { label: "Tests Processed", value: 10, suffix: "M+" },
    { label: "Healthcare Professionals", value: 5000, suffix: "+" },
    { label: "Customer Satisfaction", value: 98, suffix: "%" },
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
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 0.2 }}
            className="text-center max-w-4xl mx-auto space-y-6"
          >
            <div className="inline-block px-4 py-2 rounded-full bg-primary/10 text-primary text-sm font-medium">
              Our Story
            </div>

            <h1 className="text-5xl md:text-6xl font-bold">
              Transforming Healthcare Through <span className="gradient-text">Technology</span>
            </h1>

            <p className="text-xl text-slate-600 leading-relaxed">
              We're on a mission to modernize India's laboratory ecosystem with cloud-native technology,
              AI-powered automation, and seamless integrations that put patient care first.
            </p>
          </motion.div>
        </div>
      </section>

      {/* Stats Section */}
      <section className="section-padding bg-slate-50">
        <div className="container-wide">
          <div className="grid grid-cols-2 md:grid-cols-4 gap-8">
            {stats.map((stat, i) => (
              <motion.div
                key={i}
                initial={{ opacity: 0, y: 10 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.3, delay: i * 0.05 }}
                className="text-center"
              >
                <AnimatedCounter
                  value={stat.value}
                  suffix={stat.suffix}
                  className="text-5xl font-bold text-primary mb-2"
                />
                <p className="text-slate-600">{stat.label}</p>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Mission & Vision */}
      <section className="section-padding">
        <div className="container-wide">
          <div className="grid lg:grid-cols-2 gap-12">
            <motion.div
              initial={{ opacity: 0, x: -15 }}
              whileInView={{ opacity: 1, x: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.4 }}
            >
              <GlassCard variant="strong" className="p-8 h-full">
                <div className="p-3 rounded-xl bg-primary/10 w-fit mb-6">
                  <Target className="w-8 h-8 text-primary" />
                </div>
                <h2 className="text-3xl font-bold mb-4">Our Mission</h2>
                <p className="text-slate-600 text-lg leading-relaxed">
                  To democratize access to world-class laboratory technology for healthcare providers across India.
                  We believe every lab, regardless of size, deserves enterprise-grade tools that enhance accuracy,
                  improve efficiency, and ultimately save lives.
                </p>
              </GlassCard>
            </motion.div>

            <motion.div
              initial={{ opacity: 0, x: 15 }}
              whileInView={{ opacity: 1, x: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.4 }}
            >
              <GlassCard variant="strong" className="p-8 h-full">
                <div className="p-3 rounded-xl bg-purple-500/10 w-fit mb-6">
                  <Eye className="w-8 h-8 text-purple-600" />
                </div>
                <h2 className="text-3xl font-bold mb-4">Our Vision</h2>
                <p className="text-slate-600 text-lg leading-relaxed">
                  To be India's most trusted laboratory information system, powering 10,000+ labs by 2030.
                  We envision a future where AI-powered diagnostics, seamless data sharing, and patient-centric
                  care are the norm, not the exception.
                </p>
              </GlassCard>
            </motion.div>
          </div>
        </div>
      </section>

      {/* Core Values */}
      <section className="section-padding bg-slate-50">
        <div className="container-wide">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl font-bold mb-4">
              Our Core <span className="gradient-text">Values</span>
            </h2>
            <p className="text-xl text-slate-600 max-w-2xl mx-auto">
              These principles guide every decision we make and every line of code we write.
            </p>
          </motion.div>

          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
            {values.map((value, i) => (
              <motion.div
                key={i}
                initial={{ opacity: 0, y: 10 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.3, delay: i * 0.05 }}
              >
                <GlassCard className="p-6 h-full text-center">
                  <div className={`p-4 rounded-xl bg-${value.color}-500/10 w-fit mx-auto mb-4`}>
                    <value.icon className={`w-8 h-8 text-${value.color}-600`} />
                  </div>
                  <h3 className="text-xl font-bold mb-2">{value.title}</h3>
                  <p className="text-slate-600 text-sm">{value.description}</p>
                </GlassCard>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Timeline */}
      <section className="section-padding">
        <div className="container-wide">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl font-bold mb-4">
              Our <span className="gradient-text">Journey</span>
            </h2>
            <p className="text-xl text-slate-600 max-w-2xl mx-auto">
              From a startup to serving 500+ laboratories across India.
            </p>
          </motion.div>

          <div className="max-w-4xl mx-auto">
            <div className="relative">
              {/* Timeline Line */}
              <div className="absolute left-1/2 transform -translate-x-1/2 h-full w-0.5 bg-gradient-to-b from-primary/20 via-primary to-primary/20" />

              {milestones.map((milestone, i) => (
                <motion.div
                  key={i}
                  initial={{ opacity: 0, x: i % 2 === 0 ? -15 : 15 }}
                  whileInView={{ opacity: 1, x: 0 }}
                  viewport={{ once: true }}
                  transition={{ duration: 0.4, delay: i * 0.05 }}
                  className={`flex items-center gap-8 mb-12 ${
                    i % 2 === 0 ? 'flex-row' : 'flex-row-reverse'
                  }`}
                >
                  <div className={`w-1/2 ${i % 2 === 0 ? 'text-right' : 'text-left'}`}>
                    <GlassCard className="p-6">
                      <div className="text-2xl font-bold text-primary mb-2">{milestone.year}</div>
                      <h3 className="text-xl font-bold mb-2">{milestone.event}</h3>
                      <p className="text-slate-600 text-sm">{milestone.desc}</p>
                    </GlassCard>
                  </div>

                  <div className="relative z-10">
                    <div className="w-4 h-4 rounded-full bg-primary ring-4 ring-white shadow-lg" />
                  </div>

                  <div className="w-1/2" />
                </motion.div>
              ))}
            </div>
          </div>
        </div>
      </section>

      {/* Why Choose Us */}
      <section className="section-padding bg-slate-50">
        <div className="container-wide">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl font-bold mb-4">
              Why Labs <span className="gradient-text">Choose Us</span>
            </h2>
          </motion.div>

          <div className="grid md:grid-cols-3 gap-8">
            {[
              {
                icon: Award,
                title: "Industry Expertise",
                desc: "Built by healthcare professionals who understand lab workflows",
              },
              {
                icon: Shield,
                title: "Compliance First",
                desc: "NABL, ISO 15189:2022, ABDM, and DPDP 2023 compliant",
              },
              {
                icon: TrendingUp,
                title: "Proven ROI",
                desc: "30-60% reduction in TAT, 50% less manual errors",
              },
            ].map((item, i) => (
              <motion.div
                key={i}
                initial={{ opacity: 0, y: 10 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.3, delay: i * 0.05 }}
              >
                <GlassCard className="p-8 text-center h-full">
                  <div className="p-4 rounded-xl bg-primary/10 w-fit mx-auto mb-4">
                    <item.icon className="w-8 h-8 text-primary" />
                  </div>
                  <h3 className="text-xl font-bold mb-3">{item.title}</h3>
                  <p className="text-slate-600">{item.desc}</p>
                </GlassCard>
              </motion.div>
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
              className="space-y-6 max-w-2xl mx-auto"
            >
              <h2 className="text-4xl font-bold">
                Join the Future of Laboratory Management
              </h2>

              <p className="text-xl text-slate-600">
                Be part of the transformation. Start your 14-day free trial today.
              </p>

              <div className="flex flex-col sm:flex-row gap-4 justify-center pt-6">
                <Link href="/register">
                  <Button size="lg" className="hover-glow">
                    Start Free Trial
                  </Button>
                </Link>
                <Link href="/contact">
                  <Button size="lg" variant="outline" className="hover-lift">
                    Contact Us
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
