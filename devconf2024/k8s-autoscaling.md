# K8s workload autoscaling performance dimensions

- Zilvinas Urbonas
- Slides:
  https://pretalx.com/media/devconf-cz-2024/submissions/DFNCNH/resources/DevConf_24_Autoscaling_Dimensions_uw0OTuj.pdf

Org challenges of today

- deliver quality product
- push back one-off customers requirements
- avoid business critical incidents
- limit infra spend

Choose right

- platform / db versions ; performance may differ
- CPU architecture; CPUs are not all created equally.  Measure!

What can we do?

- forecast issues with application
- packing noisy apps together
- throttling
- latency
- OOM
- anything that could lead to disruption

What do you think you app needs?

- network throughput?

**kvisor** - real-time k8s issue and vuln scanning

Measure continually, cut often

- correct vertical sizing of your apps
- proper ammount of replicas

Resources are not infinite, site effects rae painful

- CPU limit -> throttling (lower rps)
- memory -> killed
- networ -> latency (lower rps)
- GPU -> throttling / killed

Applications have a lifespan

Engineer wrestling with FinOps

- pros: more spend visibility, financial accountability,
  transparency
- cons: extra pressure when making decisions, frugality is relative,
  potential conflicts (prioritisation, blame)

Production incidents

- make mistakes, learn from them
- keep your head

*War stories*

Tools

- mix of VPA (vertical pod autoscaling) & HPA (horizontal) solutions
- KEDA + plugins
- open and closed source autoscalers
- metrics & stress-ng
