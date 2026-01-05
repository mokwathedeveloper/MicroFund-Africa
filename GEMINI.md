# Gemini AI Memory Prompt for Cosmetics E-Commerce Project

## Project Stack
- Backend: **NestJS (TypeScript)**  
- Frontend: **Next.js + shadcn UI / Tailwind CSS / shadcn CSS**  
- Database: **Supabase (PostgreSQL)**  
- Admin Authentication: **Supabase Auth**  
- Payment Integration (future): **M-Pesa / Flutterwave**  
- Images / Storage: **Supabase Storage**  

## Functional Requirements
1. **Backend**
   - NestJS modules: Products, Categories, Orders, Admin
   - Products support: variants, images, stock, description, category
   - Admin API endpoints for CRUD operations
   - Proper DTO validation for all inputs
   - Error handling in all API routes
   - Connect all queries to **Supabase** only
2. **Frontend**
   - Admin dashboard for product and category management
   - Use **shadcn UI components** for tables, forms, modals
   - Fetch from NestJS APIs only; submit product entries through API
   - Form validation and error/success display
3. **No hallucination**
   - Only implement features described in this memory
   - Never invent features, pages, or workflows
   - Never change the stack or DB
4. **Git Workflow**
   - Commit every file after creating or modifying:  
     `git add <filename>` with descriptive commit message
   - Repeat for every change
5. **Build / Verification**
   - After implementing features, always run:
     - `npm run build` for frontend
     - `npm run build` for backend
   - Ensure no build errors before moving to next step
6. **Admin Panel Rules**
   - Admin can create, update, delete products and categories
   - Product form must include name, description, price, stock, images, variants
   - Admin dashboard shows product table with edit/delete buttons
7. **Frontend Rules**
   - Use **Tailwind + shadcn CSS** consistently for styling
   - Components must be responsive and professional
8. **Supabase Rules**
   - Store products, categories, orders in PostgreSQL tables
   - Images uploaded to Supabase Storage
   - Use Supabase Auth for admin login

## Permanent Memory Enforcement
- Remember the stack, rules, DB schema, admin requirements, and styling
- Never suggest different frameworks, libraries, or backend
- Always enforce git commit and build after any code generation
