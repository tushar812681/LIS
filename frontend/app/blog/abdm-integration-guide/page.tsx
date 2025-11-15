'use client';

import Link from "next/link";
import { motion } from "framer-motion";
import { Button } from "@/components/ui/button";
import { GlassCard } from "@/components/ui/glass-card";
import { Navbar } from "@/components/layout/Navbar";
import { Footer } from "@/components/layout/Footer";
import { ArrowLeft, Calendar, Clock, User, Share2, Bookmark, Check } from "lucide-react";

export default function ABDMIntegrationGuidePage() {
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

            <div className="inline-block px-3 py-1 rounded-full bg-green-500/10 text-green-600 text-sm font-medium mb-4">
              Integration
            </div>

            <h1 className="text-5xl md:text-6xl font-bold mb-6">
              Complete Guide to <span className="gradient-text">ABDM Integration</span>
            </h1>

            <div className="flex flex-wrap items-center gap-6 text-slate-600 mb-8">
              <div className="flex items-center gap-2">
                <Calendar className="w-4 h-4" />
                <span className="text-sm">March 10, 2025</span>
              </div>
              <div className="flex items-center gap-2">
                <Clock className="w-4 h-4" />
                <span className="text-sm">12 min read</span>
              </div>
              <div className="flex items-center gap-2">
                <User className="w-4 h-4" />
                <span className="text-sm">Rajesh Kumar</span>
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
                  The Ayushman Bharat Digital Mission (ABDM) is transforming India's healthcare ecosystem.
                  Learn everything you need to know about integrating your laboratory with ABDM and
                  becoming part of India's digital health infrastructure.
                </p>

                <h2 className="text-3xl font-bold mt-12 mb-4">What is ABDM?</h2>
                <p>
                  The Ayushman Bharat Digital Mission is the National Health Authority's flagship initiative
                  to create a digital health ecosystem in India. ABDM aims to develop the backbone necessary
                  to support the integrated digital health infrastructure of the country.
                </p>

                <h3 className="text-2xl font-bold mt-8 mb-4">Key Components of ABDM</h3>

                <div className="space-y-4 my-8">
                  <div className="flex items-start gap-3">
                    <div className="p-2 rounded-lg bg-green-500/10 flex-shrink-0">
                      <Check className="w-5 h-5 text-green-600" />
                    </div>
                    <div>
                      <h4 className="font-semibold mb-1">Health ID (ABHA)</h4>
                      <p className="text-slate-600 mb-0">
                        Unique health identifier for every Indian citizen to access and share health records digitally
                      </p>
                    </div>
                  </div>

                  <div className="flex items-start gap-3">
                    <div className="p-2 rounded-lg bg-green-500/10 flex-shrink-0">
                      <Check className="w-5 h-5 text-green-600" />
                    </div>
                    <div>
                      <h4 className="font-semibold mb-1">Healthcare Professionals Registry (HPR)</h4>
                      <p className="text-slate-600 mb-0">
                        Repository of all registered healthcare professionals with verified credentials
                      </p>
                    </div>
                  </div>

                  <div className="flex items-start gap-3">
                    <div className="p-2 rounded-lg bg-green-500/10 flex-shrink-0">
                      <Check className="w-5 h-5 text-green-600" />
                    </div>
                    <div>
                      <h4 className="font-semibold mb-1">Health Facility Registry (HFR)</h4>
                      <p className="text-slate-600 mb-0">
                        Comprehensive repository of health facilities across the country
                      </p>
                    </div>
                  </div>

                  <div className="flex items-start gap-3">
                    <div className="p-2 rounded-lg bg-green-500/10 flex-shrink-0">
                      <Check className="w-5 h-5 text-green-600" />
                    </div>
                    <div>
                      <h4 className="font-semibold mb-1">Personal Health Records (PHR)</h4>
                      <p className="text-slate-600 mb-0">
                        Secure digital health records accessible to patients and authorized healthcare providers
                      </p>
                    </div>
                  </div>
                </div>

                <h2 className="text-3xl font-bold mt-12 mb-4">Why ABDM Integration Matters for Laboratories</h2>
                <p>
                  Integrating your laboratory with ABDM provides numerous benefits:
                </p>

                <h3 className="text-2xl font-bold mt-8 mb-4">For Laboratories</h3>
                <ul>
                  <li>Access to a wider patient base through ABDM network</li>
                  <li>Streamlined digital report delivery to patients</li>
                  <li>Enhanced credibility and visibility in the digital health ecosystem</li>
                  <li>Compliance with government digital health initiatives</li>
                  <li>Reduced paperwork and administrative burden</li>
                </ul>

                <h3 className="text-2xl font-bold mt-8 mb-4">For Patients</h3>
                <ul>
                  <li>Easy access to lab reports through ABHA app</li>
                  <li>Ability to share reports with doctors digitally</li>
                  <li>Consolidated health records in one place</li>
                  <li>Improved continuity of care</li>
                </ul>

                <h2 className="text-3xl font-bold mt-12 mb-4">Integration Requirements</h2>
                <p>
                  To integrate with ABDM, laboratories must meet certain technical and regulatory requirements:
                </p>

                <h3 className="text-2xl font-bold mt-8 mb-4">Technical Requirements</h3>
                <div className="bg-slate-50 rounded-lg p-6 my-8">
                  <ul className="space-y-2 mb-0">
                    <li>ABDM-compliant Laboratory Information System (LIS)</li>
                    <li>FHIR (Fast Healthcare Interoperability Resources) support</li>
                    <li>Secure API endpoints for data exchange</li>
                    <li>SSL/TLS encryption for data transmission</li>
                    <li>ABHA verification capability</li>
                  </ul>
                </div>

                <h3 className="text-2xl font-bold mt-8 mb-4">Regulatory Requirements</h3>
                <div className="bg-slate-50 rounded-lg p-6 my-8">
                  <ul className="space-y-2 mb-0">
                    <li>Valid laboratory license/registration</li>
                    <li>NABL accreditation (recommended)</li>
                    <li>Data privacy and security compliance</li>
                    <li>Consent management framework implementation</li>
                    <li>Registration in Health Facility Registry (HFR)</li>
                  </ul>
                </div>

                <h2 className="text-3xl font-bold mt-12 mb-4">Step-by-Step Integration Process</h2>

                <h3 className="text-2xl font-bold mt-8 mb-4">Step 1: Register Your Facility</h3>
                <p>
                  Register your laboratory in the Health Facility Registry (HFR). This creates your unique
                  facility identifier and establishes your presence in the ABDM ecosystem.
                </p>

                <h3 className="text-2xl font-bold mt-8 mb-4">Step 2: Obtain ABDM Sandbox Access</h3>
                <p>
                  Apply for sandbox access through the ABDM portal. This provides a testing environment
                  where you can develop and test your integration before going live.
                </p>

                <h3 className="text-2xl font-bold mt-8 mb-4">Step 3: Implement FHIR Standards</h3>
                <p>
                  Ensure your LIS supports FHIR standards for health data exchange. This includes
                  implementing FHIR resources for diagnostic reports, observations, and patient data.
                </p>

                <h3 className="text-2xl font-bold mt-8 mb-4">Step 4: Develop Integration APIs</h3>
                <p>
                  Develop APIs that connect your LIS with ABDM building blocks:
                </p>
                <ul>
                  <li>ABHA verification API</li>
                  <li>Consent management API</li>
                  <li>Health information exchange API</li>
                  <li>Report submission API</li>
                </ul>

                <h3 className="text-2xl font-bold mt-8 mb-4">Step 5: Testing and Validation</h3>
                <p>
                  Thoroughly test your integration in the sandbox environment. Validate:
                </p>
                <ul>
                  <li>ABHA verification flow</li>
                  <li>Consent capture and management</li>
                  <li>Report generation in FHIR format</li>
                  <li>Secure data transmission</li>
                  <li>Error handling and recovery</li>
                </ul>

                <h3 className="text-2xl font-bold mt-8 mb-4">Step 6: Production Deployment</h3>
                <p>
                  Once testing is complete, apply for production credentials and deploy your integration.
                  Monitor the system closely during initial rollout.
                </p>

                <h2 className="text-3xl font-bold mt-12 mb-4">Floe-LIS: Built-in ABDM Integration</h2>
                <p>
                  Floe-LIS comes with comprehensive ABDM integration out of the box, eliminating the need
                  for complex development work. Our platform includes:
                </p>

                <div className="space-y-4 my-8">
                  <div className="bg-primary/5 border-l-4 border-primary rounded-lg p-6">
                    <h4 className="font-semibold mb-2">Pre-configured FHIR Support</h4>
                    <p className="text-slate-600 mb-0">
                      Built-in FHIR resources and profiles for seamless ABDM integration
                    </p>
                  </div>

                  <div className="bg-primary/5 border-l-4 border-primary rounded-lg p-6">
                    <h4 className="font-semibold mb-2">Automated ABHA Verification</h4>
                    <p className="text-slate-600 mb-0">
                      Instant patient verification using ABHA ID with consent management
                    </p>
                  </div>

                  <div className="bg-primary/5 border-l-4 border-primary rounded-lg p-6">
                    <h4 className="font-semibold mb-2">Digital Report Delivery</h4>
                    <p className="text-slate-600 mb-0">
                      Automatic report submission to patient's PHR upon completion
                    </p>
                  </div>

                  <div className="bg-primary/5 border-l-4 border-primary rounded-lg p-6">
                    <h4 className="font-semibold mb-2">Compliance Management</h4>
                    <p className="text-slate-600 mb-0">
                      Built-in tools for maintaining ABDM compliance and audit trails
                    </p>
                  </div>
                </div>

                <h2 className="text-3xl font-bold mt-12 mb-4">Common Challenges and Solutions</h2>

                <h3 className="text-2xl font-bold mt-8 mb-4">Challenge 1: FHIR Implementation Complexity</h3>
                <p className="text-slate-600">
                  <strong>Solution:</strong> Use platforms like Floe-LIS that provide pre-built FHIR
                  support, or engage with FHIR implementation partners who specialize in healthcare integration.
                </p>

                <h3 className="text-2xl font-bold mt-8 mb-4">Challenge 2: Data Security Concerns</h3>
                <p className="text-slate-600">
                  <strong>Solution:</strong> Implement robust encryption, access controls, and audit
                  logging. Regular security audits and penetration testing are essential.
                </p>

                <h3 className="text-2xl font-bold mt-8 mb-4">Challenge 3: Patient Consent Management</h3>
                <p className="text-slate-600">
                  <strong>Solution:</strong> Implement a clear consent workflow that's integrated into
                  your patient registration process. Ensure staff are trained on consent procedures.
                </p>

                <h2 className="text-3xl font-bold mt-12 mb-4">Future of ABDM</h2>
                <p>
                  ABDM is rapidly evolving with new features and capabilities being added regularly:
                </p>
                <ul>
                  <li>Expansion of health data sharing capabilities</li>
                  <li>Integration with insurance and claim processing</li>
                  <li>Enhanced AI-powered health insights</li>
                  <li>Telemedicine integration</li>
                  <li>Medication tracking and management</li>
                </ul>

                <div className="bg-green-500/10 border-l-4 border-green-500 rounded-lg p-6 my-8">
                  <p className="font-semibold text-lg mb-2">Ready to integrate with ABDM?</p>
                  <p className="text-slate-600 mb-0">
                    Floe-LIS makes ABDM integration simple and straightforward. Contact us today to learn
                    how we can help your laboratory become part of India's digital health revolution.
                  </p>
                </div>
              </article>
            </GlassCard>

            {/* Related Articles */}
            <div className="mt-12">
              <h3 className="text-2xl font-bold mb-6">Related Articles</h3>
              <div className="grid md:grid-cols-2 gap-6">
                <Link href="/blog/ai-laboratory-automation">
                  <GlassCard className="p-6 hover-lift cursor-pointer">
                    <div className="inline-block px-3 py-1 rounded-full bg-blue-500/10 text-blue-600 text-sm font-medium mb-3">
                      Technology
                    </div>
                    <h4 className="font-semibold mb-2">AI and Machine Learning in Laboratory Automation</h4>
                    <p className="text-sm text-slate-600">
                      How AI is transforming laboratory operations and diagnostics
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
