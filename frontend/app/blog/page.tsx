'use client';

import Link from "next/link";
import { GlassCard } from "@/components/ui/glass-card";
import { Navbar } from "@/components/layout/Navbar";
import { Footer } from "@/components/layout/Footer";
import { Calendar, Clock, ArrowRight } from "lucide-react";

export default function BlogPage() {
  const posts = [
    {
      title: "Revolutionizing Laboratory Workflows with AI",
      excerpt: "Discover how AI-powered automation is transforming diagnostic laboratories across India.",
      date: "January 5, 2025",
      readTime: "5 min read",
      category: "Technology",
      slug: "ai-laboratory-automation",
    },
    {
      title: "Understanding ABDM Integration for Labs",
      excerpt: "A comprehensive guide to integrating your laboratory with India's Ayushman Bharat Digital Mission.",
      date: "December 28, 2024",
      readTime: "8 min read",
      category: "Integration",
      slug: "abdm-integration-guide",
    },
    {
      title: "NABL Compliance Made Simple",
      excerpt: "Essential steps to achieve and maintain NABL accreditation with modern LIS technology.",
      date: "December 20, 2024",
      readTime: "6 min read",
      category: "Compliance",
      slug: "nabl-compliance-guide",
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
              Latest Updates
            </div>

            <h1 className="text-5xl md:text-6xl font-bold">
              Floe-LIS <span className="gradient-text">Blog</span>
            </h1>

            <p className="text-xl text-slate-600">
              Insights on laboratory technology, healthcare innovations, and industry best practices
            </p>
          </div>
        </div>
      </section>

      {/* Blog Posts */}
      <section className="section-padding bg-slate-50">
        <div className="container-wide max-w-5xl">
          <div className="space-y-8">
            {posts.map((post, i) => (
              <GlassCard key={i} className="p-8 hover-lift">
                <div className="space-y-4">
                  <div className="flex items-center gap-4 text-sm text-slate-600">
                    <span className="px-3 py-1 rounded-full bg-primary/10 text-primary font-medium">
                      {post.category}
                    </span>
                    <span className="flex items-center gap-2">
                      <Calendar className="w-4 h-4" />
                      {post.date}
                    </span>
                    <span className="flex items-center gap-2">
                      <Clock className="w-4 h-4" />
                      {post.readTime}
                    </span>
                  </div>

                  <h2 className="text-3xl font-bold">{post.title}</h2>

                  <p className="text-slate-600 text-lg">
                    {post.excerpt}
                  </p>

                  <Link
                    href={`/blog/${post.slug}`}
                    className="inline-flex items-center gap-2 text-primary font-medium hover:gap-3 transition-all"
                  >
                    Read Article
                    <ArrowRight className="w-4 h-4" />
                  </Link>
                </div>
              </GlassCard>
            ))}
          </div>

          {/* Coming Soon Message */}
          <GlassCard variant="strong" className="p-8 text-center mt-12">
            <p className="text-slate-600">
              More articles coming soon. Stay tuned for updates on laboratory technology and healthcare innovation.
            </p>
          </GlassCard>
        </div>
      </section>

      <Footer />
    </div>
  );
}
