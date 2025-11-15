'use client';

import Link from "next/link";
import { motion } from "framer-motion";
import { Button } from "@/components/ui/button";
import { GlassCard } from "@/components/ui/glass-card";
import { Navbar } from "@/components/layout/Navbar";
import { Footer } from "@/components/layout/Footer";
import { ArrowLeft, Calendar, Clock, User, Share2, Bookmark } from "lucide-react";

export default function AILaboratoryAutomationPage() {
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
            className="max-w-4xl mx-auto"
          >
            <Link href="/blog">
              <Button variant="ghost" className="mb-6">
                <ArrowLeft className="w-4 h-4 mr-2" />
                Back to Blog
              </Button>
            </Link>

            <div className="inline-block px-3 py-1 rounded-full bg-blue-500/10 text-blue-600 text-sm font-medium mb-4">
              Technology
            </div>

            <h1 className="text-5xl md:text-6xl font-bold mb-6">
              AI and Machine Learning in <span className="gradient-text">Laboratory Automation</span>
            </h1>

            <div className="flex flex-wrap items-center gap-6 text-slate-600 mb-8">
              <div className="flex items-center gap-2">
                <Calendar className="w-4 h-4" />
                <span className="text-sm">March 15, 2025</span>
              </div>
              <div className="flex items-center gap-2">
                <Clock className="w-4 h-4" />
                <span className="text-sm">8 min read</span>
              </div>
              <div className="flex items-center gap-2">
                <User className="w-4 h-4" />
                <span className="text-sm">Dr. Priya Sharma</span>
              </div>
            </div>

            <div className="flex gap-3">
              <Button variant="outline" size="sm">
                <Share2 className="w-4 h-4 mr-2" />
                Share
              </Button>
              <Button variant="outline" size="sm">
                <Bookmark className="w-4 h-4 mr-2" />
                Save
              </Button>
            </div>
          </motion.div>
        </div>
      </section>

      {/* Article Content */}
      <section className="py-12">
        <div className="container-wide">
          <div className="max-w-4xl mx-auto">
            <GlassCard className="p-8 md:p-12">
              <article className="prose prose-lg max-w-none">
                <p className="text-xl text-slate-600 leading-relaxed mb-8">
                  Artificial Intelligence and Machine Learning are revolutionizing laboratory operations,
                  enabling unprecedented levels of accuracy, efficiency, and predictive capabilities in
                  diagnostic testing and research.
                </p>

                <h2 className="text-3xl font-bold mt-12 mb-4">The AI Revolution in Laboratories</h2>
                <p>
                  The integration of AI and ML technologies in laboratory information systems represents
                  a paradigm shift in how laboratories operate. From automated result interpretation to
                  predictive maintenance of equipment, AI is transforming every aspect of lab operations.
                </p>

                <h3 className="text-2xl font-bold mt-8 mb-4">Key Applications</h3>

                <h4 className="text-xl font-semibold mt-6 mb-3">1. Automated Result Interpretation</h4>
                <p>
                  Machine learning models can analyze test results with remarkable accuracy, identifying
                  patterns that might be missed by human observers. These systems can:
                </p>
                <ul>
                  <li>Detect anomalies in blood cell morphology</li>
                  <li>Identify potential sample contamination</li>
                  <li>Flag critical results requiring immediate attention</li>
                  <li>Reduce false positives and negatives</li>
                </ul>

                <h4 className="text-xl font-semibold mt-6 mb-3">2. Predictive Analytics</h4>
                <p>
                  AI systems can predict equipment failures before they occur, allowing for proactive
                  maintenance and reducing downtime. This includes:
                </p>
                <ul>
                  <li>Monitoring equipment performance patterns</li>
                  <li>Predicting reagent expiration and usage</li>
                  <li>Optimizing inventory management</li>
                  <li>Forecasting testing volume and resource needs</li>
                </ul>

                <h4 className="text-xl font-semibold mt-6 mb-3">3. Quality Control Enhancement</h4>
                <p>
                  Machine learning algorithms continuously learn from quality control data, improving
                  their ability to detect deviations and ensure consistent test quality. Benefits include:
                </p>
                <ul>
                  <li>Real-time quality monitoring across all analyzers</li>
                  <li>Automated detection of systematic errors</li>
                  <li>Trend analysis for proactive quality management</li>
                  <li>Reduced manual QC review time</li>
                </ul>

                <h2 className="text-3xl font-bold mt-12 mb-4">Implementation in Floe-LIS</h2>
                <p>
                  Floe-LIS incorporates cutting-edge AI capabilities designed specifically for Indian
                  healthcare laboratories. Our system includes:
                </p>

                <div className="bg-slate-50 rounded-lg p-6 my-8">
                  <h4 className="font-semibold mb-3">Smart Result Validation</h4>
                  <p className="text-slate-600 mb-0">
                    AI-powered validation engine that learns from historical data to identify unusual
                    results, reducing the burden on laboratory staff while improving accuracy.
                  </p>
                </div>

                <div className="bg-slate-50 rounded-lg p-6 my-8">
                  <h4 className="font-semibold mb-3">Predictive Maintenance</h4>
                  <p className="text-slate-600 mb-0">
                    Machine learning models monitor equipment performance and predict maintenance needs,
                    helping laboratories avoid costly downtime and ensure continuous operations.
                  </p>
                </div>

                <div className="bg-slate-50 rounded-lg p-6 my-8">
                  <h4 className="font-semibold mb-3">Intelligent Workflow Optimization</h4>
                  <p className="text-slate-600 mb-0">
                    AI algorithms analyze laboratory workflows to identify bottlenecks and suggest
                    optimizations, improving turnaround times and resource utilization.
                  </p>
                </div>

                <h2 className="text-3xl font-bold mt-12 mb-4">Real-World Impact</h2>
                <p>
                  Laboratories implementing AI-powered systems like Floe-LIS have reported significant improvements:
                </p>
                <ul>
                  <li>40% reduction in result verification time</li>
                  <li>60% decrease in equipment downtime</li>
                  <li>30% improvement in turnaround times</li>
                  <li>25% reduction in reagent waste</li>
                  <li>Enhanced compliance with quality standards</li>
                </ul>

                <h2 className="text-3xl font-bold mt-12 mb-4">The Future of AI in Laboratories</h2>
                <p>
                  As AI and ML technologies continue to evolve, we can expect even more transformative
                  applications in laboratory medicine:
                </p>
                <ul>
                  <li>Advanced diagnostic support with personalized treatment recommendations</li>
                  <li>Integration with genomic data for precision medicine</li>
                  <li>Automated protocol optimization based on patient demographics</li>
                  <li>Real-time collaboration with AI-powered clinical decision support</li>
                </ul>

                <h2 className="text-3xl font-bold mt-12 mb-4">Getting Started</h2>
                <p>
                  Implementing AI in your laboratory doesn't have to be overwhelming. Floe-LIS makes it
                  easy to leverage AI capabilities without requiring extensive technical expertise. Our
                  platform provides:
                </p>
                <ul>
                  <li>Pre-trained models optimized for Indian laboratory workflows</li>
                  <li>Intuitive interfaces that require no coding knowledge</li>
                  <li>Comprehensive training and support</li>
                  <li>Seamless integration with existing laboratory equipment</li>
                </ul>

                <div className="bg-primary/5 border-l-4 border-primary rounded-lg p-6 my-8">
                  <p className="font-semibold text-lg mb-2">Ready to transform your laboratory?</p>
                  <p className="text-slate-600 mb-0">
                    Contact us today to learn how Floe-LIS can bring the power of AI and machine
                    learning to your laboratory operations.
                  </p>
                </div>
              </article>
            </GlassCard>

            {/* Related Articles */}
            <div className="mt-12">
              <h3 className="text-2xl font-bold mb-6">Related Articles</h3>
              <div className="grid md:grid-cols-2 gap-6">
                <Link href="/blog/abdm-integration-guide">
                  <GlassCard className="p-6 hover-lift cursor-pointer">
                    <div className="inline-block px-3 py-1 rounded-full bg-green-500/10 text-green-600 text-sm font-medium mb-3">
                      Integration
                    </div>
                    <h4 className="font-semibold mb-2">Complete Guide to ABDM Integration</h4>
                    <p className="text-sm text-slate-600">
                      Everything you need to know about Ayushman Bharat Digital Mission integration
                    </p>
                  </GlassCard>
                </Link>

                <Link href="/blog/nabl-compliance-guide">
                  <GlassCard className="p-6 hover-lift cursor-pointer">
                    <div className="inline-block px-3 py-1 rounded-full bg-purple-500/10 text-purple-600 text-sm font-medium mb-3">
                      Compliance
                    </div>
                    <h4 className="font-semibold mb-2">NABL Compliance Made Easy</h4>
                    <p className="text-sm text-slate-600">
                      Step-by-step guide to achieving and maintaining NABL accreditation
                    </p>
                  </GlassCard>
                </Link>
              </div>
            </div>
          </div>
        </div>
      </section>

      <Footer />
    </div>
  );
}
