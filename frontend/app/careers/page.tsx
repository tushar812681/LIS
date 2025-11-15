'use client';

import Link from "next/link";
import { Button } from "@/components/ui/button";
import { GlassCard } from "@/components/ui/glass-card";
import { Navbar } from "@/components/layout/Navbar";
import { Footer } from "@/components/layout/Footer";
import { Briefcase, MapPin, Clock, Users, Heart, Zap, TrendingUp } from "lucide-react";

export default function CareersPage() {
  const positions = [
    {
      title: "Senior Backend Engineer (Rust)",
      department: "Engineering",
      location: "Bangalore / Remote",
      type: "Full-time",
      description: "Build scalable backend systems for healthcare infrastructure",
    },
    {
      title: "Frontend Developer (Next.js)",
      department: "Engineering",
      location: "Bangalore / Remote",
      type: "Full-time",
      description: "Create beautiful, performant user interfaces for healthcare professionals",
    },
    {
      title: "Product Manager - Healthcare",
      department: "Product",
      location: "Bangalore",
      type: "Full-time",
      description: "Drive product strategy for laboratory information systems",
    },
  ];

  const benefits = [
    {
      icon: Heart,
      title: "Health & Wellness",
      description: "Comprehensive health insurance for you and your family",
    },
    {
      icon: TrendingUp,
      title: "Growth",
      description: "Learning budget and career development opportunities",
    },
    {
      icon: Users,
      title: "Team Culture",
      description: "Collaborative environment with talented healthcare tech experts",
    },
    {
      icon: Zap,
      title: "Impact",
      description: "Work on technology that transforms healthcare in India",
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
          <div className="text-center max-w-3xl mx-auto space-y-6">
            <div className="inline-block px-4 py-2 rounded-full bg-primary/10 text-primary text-sm font-medium">
              Join Our Team
            </div>

            <h1 className="text-5xl md:text-6xl font-bold">
              Build the Future of <span className="gradient-text">Healthcare Tech</span>
            </h1>

            <p className="text-xl text-slate-600">
              Help us transform laboratory management and improve healthcare outcomes across India
            </p>
          </div>
        </div>
      </section>

      {/* Benefits */}
      <section className="section-padding bg-slate-50">
        <div className="container-wide">
          <div className="text-center mb-12">
            <h2 className="text-4xl font-bold mb-4">
              Why <span className="gradient-text">Floe-LIS</span>?
            </h2>
          </div>

          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
            {benefits.map((benefit, i) => (
              <GlassCard key={i} className="p-6 text-center">
                <div className="p-3 rounded-xl bg-primary/10 w-fit mx-auto mb-4">
                  <benefit.icon className="w-8 h-8 text-primary" />
                </div>
                <h3 className="font-bold mb-2">{benefit.title}</h3>
                <p className="text-slate-600 text-sm">{benefit.description}</p>
              </GlassCard>
            ))}
          </div>
        </div>
      </section>

      {/* Open Positions */}
      <section className="section-padding">
        <div className="container-wide max-w-4xl">
          <div className="text-center mb-12">
            <h2 className="text-4xl font-bold mb-4">
              Open <span className="gradient-text">Positions</span>
            </h2>
            <p className="text-xl text-slate-600">
              Join our mission to modernize healthcare technology
            </p>
          </div>

          <div className="space-y-6">
            {positions.map((position, i) => (
              <GlassCard key={i} className="p-8 hover-lift">
                <div className="space-y-4">
                  <div className="flex items-start justify-between">
                    <div>
                      <h3 className="text-2xl font-bold mb-2">{position.title}</h3>
                      <p className="text-slate-600">{position.description}</p>
                    </div>
                  </div>

                  <div className="flex flex-wrap gap-4 text-sm text-slate-600">
                    <span className="flex items-center gap-2">
                      <Briefcase className="w-4 h-4" />
                      {position.department}
                    </span>
                    <span className="flex items-center gap-2">
                      <MapPin className="w-4 h-4" />
                      {position.location}
                    </span>
                    <span className="flex items-center gap-2">
                      <Clock className="w-4 h-4" />
                      {position.type}
                    </span>
                  </div>

                  <div className="pt-4">
                    <Link href="/contact">
                      <Button className="hover-glow">Apply Now</Button>
                    </Link>
                  </div>
                </div>
              </GlassCard>
            ))}
          </div>

          <GlassCard variant="strong" className="p-8 text-center mt-12">
            <h3 className="text-2xl font-bold mb-4">Don't see a role that fits?</h3>
            <p className="text-slate-600 mb-6">
              We're always looking for talented individuals. Send us your resume!
            </p>
            <Link href="/contact">
              <Button size="lg" className="hover-glow">
                Get in Touch
              </Button>
            </Link>
          </GlassCard>
        </div>
      </section>

      <Footer />
    </div>
  );
}
