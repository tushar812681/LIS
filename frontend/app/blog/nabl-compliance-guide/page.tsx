'use client';

import Link from "next/link";
import { motion } from "framer-motion";
import { Button } from "@/components/ui/button";
import { GlassCard } from "@/components/ui/glass-card";
import { Navbar } from "@/components/layout/Navbar";
import { Footer } from "@/components/layout/Footer";
import { ArrowLeft, Calendar, Clock, User, Share2, Bookmark, CheckCircle } from "lucide-react";

export default function NABLComplianceGuidePage() {
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

            <div className="inline-block px-3 py-1 rounded-full bg-purple-500/10 text-purple-600 text-sm font-medium mb-4">
              Compliance
            </div>

            <h1 className="text-5xl md:text-6xl font-bold mb-6">
              <span className="gradient-text">NABL Compliance</span> Made Easy
            </h1>

            <div className="flex flex-wrap items-center gap-6 text-slate-600 mb-8">
              <div className="flex items-center gap-2">
                <Calendar className="w-4 h-4" />
                <span className="text-sm">March 5, 2025</span>
              </div>
              <div className="flex items-center gap-2">
                <Clock className="w-4 h-4" />
                <span className="text-sm">10 min read</span>
              </div>
              <div className="flex items-center gap-2">
                <User className="w-4 h-4" />
                <span className="text-sm">Dr. Anjali Mehta</span>
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
                  NABL accreditation is the gold standard for laboratory quality in India. This comprehensive
                  guide will walk you through everything you need to know about achieving and maintaining
                  NABL compliance for your laboratory.
                </p>

                <h2 className="text-3xl font-bold mt-12 mb-4">What is NABL?</h2>
                <p>
                  The National Accreditation Board for Testing and Calibration Laboratories (NABL) is the
                  premier accreditation body for laboratories in India. Established by the Department of
                  Science and Technology, NABL operates in accordance with ISO/IEC 17011:2017 and is a
                  signatory to international mutual recognition arrangements.
                </p>

                <h3 className="text-2xl font-bold mt-8 mb-4">Why NABL Accreditation Matters</h3>
                <p>
                  NABL accreditation demonstrates that your laboratory:
                </p>
                <ul>
                  <li>Operates to the highest international standards (ISO 15189:2022 for medical laboratories)</li>
                  <li>Maintains consistent quality and reliability in testing</li>
                  <li>Follows documented procedures and quality management systems</li>
                  <li>Employs competent personnel with appropriate qualifications</li>
                  <li>Uses validated methods and properly calibrated equipment</li>
                </ul>

                <h2 className="text-3xl font-bold mt-12 mb-4">Benefits of NABL Accreditation</h2>

                <div className="space-y-4 my-8">
                  <div className="bg-purple-50 rounded-lg p-6">
                    <h4 className="font-semibold mb-2 flex items-center gap-2">
                      <CheckCircle className="w-5 h-5 text-purple-600" />
                      Enhanced Credibility
                    </h4>
                    <p className="text-slate-600 mb-0">
                      NABL accreditation is recognized globally, enhancing your laboratory's reputation
                      and credibility among patients and healthcare providers.
                    </p>
                  </div>

                  <div className="bg-purple-50 rounded-lg p-6">
                    <h4 className="font-semibold mb-2 flex items-center gap-2">
                      <CheckCircle className="w-5 h-5 text-purple-600" />
                      Competitive Advantage
                    </h4>
                    <p className="text-slate-600 mb-0">
                      Many hospitals, insurance companies, and government programs require NABL
                      accreditation, opening up new business opportunities.
                    </p>
                  </div>

                  <div className="bg-purple-50 rounded-lg p-6">
                    <h4 className="font-semibold mb-2 flex items-center gap-2">
                      <CheckCircle className="w-5 h-5 text-purple-600" />
                      Quality Improvement
                    </h4>
                    <p className="text-slate-600 mb-0">
                      The accreditation process identifies areas for improvement and establishes
                      robust quality management systems.
                    </p>
                  </div>

                  <div className="bg-purple-50 rounded-lg p-6">
                    <h4 className="font-semibold mb-2 flex items-center gap-2">
                      <CheckCircle className="w-5 h-5 text-purple-600" />
                      Regulatory Compliance
                    </h4>
                    <p className="text-slate-600 mb-0">
                      NABL accreditation helps meet regulatory requirements and prepares you for
                      inspections by health authorities.
                    </p>
                  </div>
                </div>

                <h2 className="text-3xl font-bold mt-12 mb-4">ISO 15189:2022 Requirements</h2>
                <p>
                  Medical laboratories seeking NABL accreditation must comply with ISO 15189:2022, which
                  specifies requirements for quality and competence. Key areas include:
                </p>

                <h3 className="text-2xl font-bold mt-8 mb-4">1. Management Requirements</h3>
                <ul>
                  <li>Quality management system documentation</li>
                  <li>Document and record control</li>
                  <li>Service agreements with clients</li>
                  <li>Review of requests and contracts</li>
                  <li>Management review processes</li>
                  <li>Continual improvement mechanisms</li>
                  <li>Corrective and preventive actions</li>
                </ul>

                <h3 className="text-2xl font-bold mt-8 mb-4">2. Technical Requirements</h3>
                <ul>
                  <li>Personnel qualifications and competency</li>
                  <li>Accommodation and environmental conditions</li>
                  <li>Laboratory equipment and calibration</li>
                  <li>Pre-examination, examination, and post-examination procedures</li>
                  <li>Method validation and verification</li>
                  <li>Quality assurance of results</li>
                  <li>Report generation and release</li>
                </ul>

                <h2 className="text-3xl font-bold mt-12 mb-4">The Accreditation Process</h2>

                <h3 className="text-2xl font-bold mt-8 mb-4">Phase 1: Application and Documentation</h3>
                <div className="bg-slate-50 rounded-lg p-6 my-6">
                  <p className="mb-3">
                    <strong>Timeline:</strong> 2-3 months
                  </p>
                  <ul className="space-y-2 mb-0">
                    <li>Submit application to NABL with required fees</li>
                    <li>Prepare quality manual documenting your QMS</li>
                    <li>Develop standard operating procedures (SOPs)</li>
                    <li>Create work instructions and forms</li>
                    <li>Establish record-keeping systems</li>
                  </ul>
                </div>

                <h3 className="text-2xl font-bold mt-8 mb-4">Phase 2: Internal Preparation</h3>
                <div className="bg-slate-50 rounded-lg p-6 my-6">
                  <p className="mb-3">
                    <strong>Timeline:</strong> 3-6 months
                  </p>
                  <ul className="space-y-2 mb-0">
                    <li>Implement documented procedures</li>
                    <li>Train all personnel on SOPs and quality requirements</li>
                    <li>Conduct method validation studies</li>
                    <li>Participate in external quality assurance programs</li>
                    <li>Perform internal audits</li>
                    <li>Conduct management review</li>
                  </ul>
                </div>

                <h3 className="text-2xl font-bold mt-8 mb-4">Phase 3: Assessment</h3>
                <div className="bg-slate-50 rounded-lg p-6 my-6">
                  <p className="mb-3">
                    <strong>Timeline:</strong> 1-2 months
                  </p>
                  <ul className="space-y-2 mb-0">
                    <li>NABL schedules on-site assessment</li>
                    <li>Assessors review documentation and observe operations</li>
                    <li>Technical competency is evaluated</li>
                    <li>Non-conformities are identified and documented</li>
                    <li>Laboratory responds to findings with corrective actions</li>
                  </ul>
                </div>

                <h3 className="text-2xl font-bold mt-8 mb-4">Phase 4: Accreditation</h3>
                <div className="bg-slate-50 rounded-lg p-6 my-6">
                  <p className="mb-3">
                    <strong>Timeline:</strong> 1-2 months
                  </p>
                  <ul className="space-y-2 mb-0">
                    <li>NABL reviews assessment report and responses</li>
                    <li>Accreditation committee makes decision</li>
                    <li>Certificate is issued (valid for 2 years)</li>
                    <li>Laboratory is listed in NABL directory</li>
                  </ul>
                </div>

                <h2 className="text-3xl font-bold mt-12 mb-4">Maintaining NABL Accreditation</h2>
                <p>
                  Accreditation is not a one-time achievement but an ongoing commitment to quality.
                  Laboratories must:
                </p>
                <ul>
                  <li>Undergo surveillance assessments every 12 months</li>
                  <li>Renew accreditation every 2 years</li>
                  <li>Participate in proficiency testing programs</li>
                  <li>Maintain records of all quality activities</li>
                  <li>Report significant changes to NABL</li>
                  <li>Investigate and resolve complaints</li>
                  <li>Implement continuous improvement initiatives</li>
                </ul>

                <h2 className="text-3xl font-bold mt-12 mb-4">Common Challenges and Solutions</h2>

                <h3 className="text-2xl font-bold mt-8 mb-4">Challenge 1: Documentation Overload</h3>
                <p>
                  <strong>Problem:</strong> Creating and maintaining extensive documentation can be overwhelming.
                </p>
                <p className="text-slate-600">
                  <strong>Solution:</strong> Use a digital LIS like Floe-LIS that includes built-in NABL
                  documentation templates and automated record-keeping. This reduces manual effort and ensures
                  compliance.
                </p>

                <h3 className="text-2xl font-bold mt-8 mb-4">Challenge 2: Quality Control Management</h3>
                <p>
                  <strong>Problem:</strong> Tracking QC data across multiple analyzers and methods is complex.
                </p>
                <p className="text-slate-600">
                  <strong>Solution:</strong> Implement automated QC data collection and analysis. Floe-LIS
                  automatically captures QC results, generates Levey-Jennings charts, and alerts staff to
                  out-of-control situations.
                </p>

                <h3 className="text-2xl font-bold mt-8 mb-4">Challenge 3: Method Validation</h3>
                <p>
                  <strong>Problem:</strong> Conducting proper validation studies requires significant effort.
                </p>
                <p className="text-slate-600">
                  <strong>Solution:</strong> Use validation protocols and statistical tools provided by your
                  LIS. Floe-LIS includes validation study templates and automated calculation of validation
                  parameters.
                </p>

                <h3 className="text-2xl font-bold mt-8 mb-4">Challenge 4: Staff Training</h3>
                <p>
                  <strong>Problem:</strong> Ensuring all staff are trained and competent requires ongoing effort.
                </p>
                <p className="text-slate-600">
                  <strong>Solution:</strong> Maintain a training matrix and competency assessment records in
                  your LIS. Floe-LIS tracks training completion, schedules refresher training, and documents
                  competency assessments.
                </p>

                <h2 className="text-3xl font-bold mt-12 mb-4">How Floe-LIS Supports NABL Compliance</h2>
                <p>
                  Floe-LIS is designed with NABL compliance in mind, providing comprehensive tools to
                  simplify the accreditation process:
                </p>

                <div className="space-y-4 my-8">
                  <div className="bg-primary/5 border-l-4 border-primary rounded-lg p-6">
                    <h4 className="font-semibold mb-2">Built-in Quality Manual Templates</h4>
                    <p className="text-slate-600 mb-0">
                      Pre-configured quality manual and SOP templates aligned with ISO 15189:2022
                      requirements, saving months of documentation work.
                    </p>
                  </div>

                  <div className="bg-primary/5 border-l-4 border-primary rounded-lg p-6">
                    <h4 className="font-semibold mb-2">Automated QC Management</h4>
                    <p className="text-slate-600 mb-0">
                      Real-time QC data collection, Westgard rules implementation, and automated
                      Levey-Jennings chart generation.
                    </p>
                  </div>

                  <div className="bg-primary/5 border-l-4 border-primary rounded-lg p-6">
                    <h4 className="font-semibold mb-2">Comprehensive Audit Trails</h4>
                    <p className="text-slate-600 mb-0">
                      Complete audit trail of all activities, including who did what and when, meeting
                      NABL traceability requirements.
                    </p>
                  </div>

                  <div className="bg-primary/5 border-l-4 border-primary rounded-lg p-6">
                    <h4 className="font-semibold mb-2">Equipment Management</h4>
                    <p className="text-slate-600 mb-0">
                      Track calibration, maintenance, and performance monitoring for all laboratory
                      equipment with automated reminders.
                    </p>
                  </div>

                  <div className="bg-primary/5 border-l-4 border-primary rounded-lg p-6">
                    <h4 className="font-semibold mb-2">Proficiency Testing Integration</h4>
                    <p className="text-slate-600 mb-0">
                      Document PT participation, results, and corrective actions in one centralized
                      location.
                    </p>
                  </div>

                  <div className="bg-primary/5 border-l-4 border-primary rounded-lg p-6">
                    <h4 className="font-semibold mb-2">Training and Competency</h4>
                    <p className="text-slate-600 mb-0">
                      Maintain training records, competency assessments, and qualification matrices
                      for all personnel.
                    </p>
                  </div>
                </div>

                <h2 className="text-3xl font-bold mt-12 mb-4">Success Stories</h2>
                <div className="bg-slate-50 rounded-lg p-6 my-8">
                  <p className="italic mb-3">
                    "Implementing Floe-LIS reduced our NABL preparation time by 40%. The built-in
                    templates and automated QC management made compliance much easier to maintain."
                  </p>
                  <p className="font-semibold text-sm text-slate-600 mb-0">
                    - Dr. Ramesh Patel, Lab Director, MediPath Diagnostics, Mumbai
                  </p>
                </div>

                <h2 className="text-3xl font-bold mt-12 mb-4">Getting Started</h2>
                <p>
                  The journey to NABL accreditation may seem daunting, but with the right tools and
                  approach, it's entirely achievable. Key steps to get started:
                </p>
                <ol>
                  <li>Conduct a gap analysis against ISO 15189:2022 requirements</li>
                  <li>Develop a project plan with timelines and responsibilities</li>
                  <li>Invest in a compliant LIS like Floe-LIS</li>
                  <li>Train staff on quality requirements and procedures</li>
                  <li>Implement quality systems systematically</li>
                  <li>Conduct internal audits before applying</li>
                  <li>Submit application when ready for assessment</li>
                </ol>

                <div className="bg-purple-500/10 border-l-4 border-purple-500 rounded-lg p-6 my-8">
                  <p className="font-semibold text-lg mb-2">Ready to start your NABL journey?</p>
                  <p className="text-slate-600 mb-0">
                    Floe-LIS can significantly simplify your path to NABL accreditation. Contact us today
                    for a demo and learn how we can support your quality journey.
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
              </div>
            </div>
          </div>
        </div>
      </section>

      <Footer />
    </div>
  );
}
